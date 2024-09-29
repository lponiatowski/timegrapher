use anyhow::{anyhow, Context, Result};
use plotly::common::ColorScale;
use core::fmt;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, 
    Host,
    HostId, 
    Stream, 
    SupportedStreamConfig};
use std::{
    sync::Arc,
    collections::HashMap
};
use tokio::{
    time::{self, Duration},
    sync::{mpsc, Mutex},
};
use futures::stream::{self, Stream as FuturStream, StreamExt};
use std::pin::Pin;


pub fn get_connectors() -> Result<Vec<Connector>> {
    // get available hosts
    let host_ids: Vec<HostId> = cpal::available_hosts();

    // populate connectors
    let connectors: Result<Vec<Connector>> = host_ids
        .iter()
        .map(|id| {
            Connector::new(*id).context(format!("Can not detect audio connectors due to {:?}", id))
        })
        .collect();

    connectors
}

pub struct Connector {
    host: Host,
    devices: HashMap<String, Device>,
}

impl Connector {
    pub fn new(id: HostId) -> Result<Self> {
        let host = cpal::host_from_id(id)?;
        let mut devs = HashMap::new();

        for dev in host.input_devices()? {
            devs.insert(dev.name()?, dev);
        }

        Ok(Connector {
            host: host,
            devices: devs,
        })
    }

    pub fn list_device_names(&self) -> Option<Vec<String>>{
        match self.devices.len(){
            0 => None,
            _ => Some(self.devices.keys().into_iter().map(|k| k.clone()).collect::<Vec<String>>())
        }
    }

    fn get_stream_conf(&self, name: &String) -> Result<(Device, SupportedStreamConfig)> {
        // let dev: Option<Device> = self.devices.remove(name);
        let dev: Option<Device> = self.devices.get(name).cloned();

        match dev {
            Some(d) => match d.default_input_config() {
                Ok(con) => Ok((d, con)),
                Err(e) => Err(anyhow!("Unable to obtain configs for {:} due to {:}", name, e)),
            },
            None => Err(anyhow!("Unable to find device: {}", name)),
        }
    }
}

impl fmt::Display for Connector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Connector {{ host: {:} }} with {:} devices",
            self.host.id().name(),
            self.devices.len()
        )
    }
}

impl fmt::Debug for Connector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let device_keys: Vec<&String> = self.devices.keys().collect();
        write!(
            f,
            "Connector {{ host: {:}, devices: {:?} }}",
            self.host.id().name(),
            device_keys
        )
    }
}



pub struct AudioStreamBuilder {
    samplerate: f64,
    samplebuff: mpsc::Receiver<(f64,f64)>,
    stream: Stream,
}

impl AudioStreamBuilder {
    pub fn new(con: &Connector, dev: &String) -> Result<Self> {
        // here we start creation of the new stream from the connector with a device named ...
        let (dev, conf) = con.get_stream_conf(dev)?;
        let samplerate: f64 = conf.sample_rate().0 as f64;

        // // create buffer which will store the data and clone to give to the sampling function
        // let buffer: Arc<Mutex<Vec<(f64, f64)>>> = Arc::new(Mutex::new(Vec::new()));
        // let clone_buff: Arc<Mutex<Vec<(f64, f64)>>> = Arc::clone(&buffer);
        
        let (sender, receiver) = mpsc::channel(10000);

        // define error callback for the stream
        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

        let mut last_time: f64 = 0.0;  // Track the time globally
        // start streating stream based off the sample format
        let stream = match conf.sample_format() {
            cpal::SampleFormat::I8 => dev.build_input_stream(
                &conf.into(),
                move |data, _: &_| {
                    // AudioStreamBuilder::sample_collector::<i8>(data, &clone_buff, samplerate)
                    AudioStreamBuilder::sample_collector::<i8>(data, sender.clone(), samplerate, &mut last_time)
                },
                err_fn,
                None,
            )?,
            cpal::SampleFormat::I16 => dev.build_input_stream(
                &conf.into(),
                move |data, _: &_| {
                    AudioStreamBuilder::sample_collector::<i16>(data, sender.clone(), samplerate, &mut last_time)
                },
                err_fn,
                None,
            )?,
            cpal::SampleFormat::I32 => dev.build_input_stream(
                &conf.into(),
                move |data, _: &_| {
                    AudioStreamBuilder::sample_collector::<i32>(data, sender.clone(), samplerate, &mut last_time)
                },
                err_fn,
                None,
            )?,
            cpal::SampleFormat::F32 => dev.build_input_stream(
                &conf.into(),
                move |data, _: &_| {
                    AudioStreamBuilder::sample_collector::<f32>(data, sender.clone(), samplerate, &mut last_time)
                },
                err_fn,
                None,
            )?,
            sample_format => {
                return Err(anyhow::Error::msg(format!(
                    "Unsupported sample format '{sample_format}'"
                )))
            }
        };

        // finally output the AudioStreamBuilder
        Ok(Self {
            samplerate: samplerate,
            samplebuff: receiver,
            stream: stream,
        })
    }

    // this is the sampling function
    fn sample_collector<T>(data: &[T], sender: mpsc::Sender<(f64, f64)>, samplerate: f64, last_time: &mut f64)
    where
        T: cpal::Sample + Into<f64>,
    {
        for sample in data.iter() {
            *last_time += 1.0 / samplerate;
            let value = (*sample).into() as f64;

            if !sender.is_closed(){
                // Attempt to send the data if chanel is closed break out 
                let _ = sender.try_send((*last_time, value));
            } else { break; }
        }
    }

    pub fn build(self) -> Result<AudioStream>{
        // this function starts "listening" to the input and created data stream 
        // the last value is kept so to ensure the continuity in the saple timestamps
        self.stream.play()?;
        let receiver = self.samplebuff;
        
        let outputstream = stream::unfold(receiver, |mut receiver| async move {
            match receiver.recv().await {
                Some(sample) => Some((sample, receiver)),
                None => None,
            }
        });

        Ok(AudioStream {
            samplerate: self.samplerate,
            stream: Arc::new(Mutex::new(Box::pin(outputstream)))
        })
    }

    pub fn samplerate(&self) -> f64 {
        self.samplerate
    }
}


pub struct AudioStream {
    samplerate: f64,
    stream: Arc<Mutex<Pin<Box<dyn FuturStream<Item = (f64, f64)> + Send>>>>
}

impl AudioStream {
    pub fn samplerate(self) -> f64 {
        self.samplerate.clone()
    }

    pub async fn get_track_by_duration(&self, duration: f64) -> AudioTrack{
        let audiocopy: Arc<Mutex<Pin<Box<dyn FuturStream<Item = (f64, f64)> + Send>>>> = Arc::clone(&self.stream);

        let track: Arc<Mutex<Vec<(f64, f64)>>> = Arc::new(Mutex::new(Vec::new()));
        let track_c = Arc::clone(&track);
        let sr = self.samplerate;

        let _ = tokio::spawn(async move {

            let mut stream = audiocopy.lock().await;                                                    
            let mut track = track_c.lock().await;
            
            let mut local_time: f64 = 0.0.into();
            while let Some((time, value)) = stream.next().await {
                // set up braking 
                local_time += 1.0 / sr;
                if local_time > duration {
                    break;
                }
                track.push((time, value));
            }

        }).await;

        let samplerate = self.samplerate.clone();
        let track: Vec<(f64,f64)> = track.lock().await.to_vec();

        AudioTrack{
            samplerate,
            track
        }
    }

    pub fn get_stream(&self) -> Arc<Mutex<Pin<Box<dyn FuturStream<Item = (f64, f64)> + Send>>>> {
        Arc::clone(&self.stream)
    }
}

#[derive(Debug, Clone)]
pub struct AudioTrack{
    pub samplerate: f64,
    pub track: Vec<(f64, f64)>,
}

impl Default for AudioTrack {
    fn default() -> Self {
        Self {
            samplerate: 0.0.into(),
            track: Vec::new()
        }
    }
}

impl AudioTrack{

    pub fn new() -> Self {
        Self {
            samplerate: 0.0.into(),
            track: Vec::new()
        }
    }

    pub fn get_time(&self) -> Vec<f64>{
        let data = self.track.clone();
        data.iter().map(|(t, _)| *t).collect()
    }

    pub fn get_volume(&self) -> Vec<f64> {
        let data = self.track.clone();
        data.iter().map(|(_, v)| *v).collect()
    }

    pub fn get_sample_rate(&self) -> f64 {
        self.samplerate
    }

    pub fn from_rate_track(samplerate: f64, track: Vec<(f64, f64)>) -> Self {
        AudioTrack{
            samplerate,
            track
        }
    }
}