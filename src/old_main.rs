use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Host, HostId};
use std::{
    fs::File,
    io::{stdin, Write},
    sync::{Arc, Mutex},
    thread,
    time::Duration
};
use futures::{stream::StreamExt, pin_mut};
use tokio::runtime::Runtime;

// define new audio track ctructure
struct AudioTrack {
    bitrate: u32,
    track: Vec<(f64, f64)>,
}

impl AudioTrack {
    pub fn new(bitrate: u32) -> Self {
        AudioTrack {
            bitrate,
            track: Vec::new(),
        }
    }
    pub fn add_sample(&mut self, time: f64, value: f64) {
        self.track.push((time, value));
    }
    pub fn get_track(&self) -> &Vec<(f64, f64)> {
        &self.track
    }
    pub fn get_duration(self) -> f64 {
        self.track.len() as f64 / self.bitrate as f64
    }
    pub fn get_min_max(self) -> (f64, f64) {
        let min = self
            .track
            .iter()
            .map(|(_, value)| *value)
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0);
        let max = self
            .track
            .iter()
            .map(|(_, value)| *value)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0);
        (min, max)
    }
}

fn main() -> Result<()> {

    // define some initial variables
    let avail_hosts: Vec<HostId> = cpal::available_hosts();
    let mut input_line: String = String::new();

    // start host selection prompt
    println!("Please select a host to use:");
    avail_hosts.iter().enumerate().for_each(|(ind, host_id)| {
        println!("[{:#}] {:?}:", ind, host_id);
        //     if let Ok(conf) = device.default_input_config() {
        //         println!("  [{:#}-{:#}] {:?}:", host_ind, dev_ind, device.name());
        //         println!("    Default input stream config:\n      {:?}", conf);
        //     }
        // });
    });
    // get user input
    stdin().read_line(&mut input_line)?;
    let input_id = input_line.trim().parse::<usize>()?;

    println!("Selected host: {:?}", &avail_hosts[input_id]);
    let host: Host = cpal::host_from_id(avail_hosts[input_id])?;

    // start device selection prompt
    let mut avail_inputs: Vec<Device> = Vec::new();
    println!("Please select an input device to use:");
    for (ind, device) in host.input_devices()?.enumerate() {
        println!("[{:#}] {:?}:", ind, device.name());
        avail_inputs.push(device);
    }

    // get user input
    input_line.clear();
    stdin().read_line(&mut input_line)?;
    let input_id = input_line.trim().parse::<usize>()?;
    let input_device = avail_inputs.remove(input_id);

    println!("Selected input device: {:?}", &input_device.name());

    // get device configuration
    let input_config = input_device.default_input_config()?;
    println!("Default input stream config:\n{:?}", input_config);

    // start stream creation
    // create output audio track
    let audiotrack = Arc::new(Mutex::new(AudioTrack::new(input_config.sample_rate().0)));
    let audiotrack_clone = Arc::clone(&audiotrack);

    // set error call back function
    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = match input_config.sample_format() {
        cpal::SampleFormat::I8 => input_device.build_input_stream(
            &input_config.into(),
            move |data, _: &_| sample_collector::<i8>(data, &audiotrack_clone),
            err_fn,
            None,
        )?,
        cpal::SampleFormat::I16 => input_device.build_input_stream(
            &input_config.into(),
            move |data, _: &_| sample_collector::<i16>(data, &audiotrack_clone),
            err_fn,
            None,
        )?,
        cpal::SampleFormat::I32 => input_device.build_input_stream(
            &input_config.into(),
            move |data, _: &_| sample_collector::<i32>(data, &audiotrack_clone),
            err_fn,
            None,
        )?,
        cpal::SampleFormat::F32 => input_device.build_input_stream(
            &input_config.into(),
            move |data, _: &_| sample_collector::<f32>(data, &audiotrack_clone),
            err_fn,
            None,
        )?,
        sample_format => {
            return Err(anyhow::Error::msg(format!(
                "Unsupported sample format '{sample_format}'"
            )))
        }
    };

    // Start the stream
    stream.play()?;

    // Run for a certain duration to capture audio
    std::thread::sleep(std::time::Duration::from_secs(5));

    // Stop the stream
    drop(stream);

    // Print the audio track
    let result = audiotrack.lock().unwrap();
    let auidiofile = File::create("audiofile.txt")?;
    let mut writer = std::io::BufWriter::new(auidiofile);
    for (time, value) in result.get_track().iter() {
        writeln!(writer, "{},{}", time, value)?;
    }
    writer.flush()?;
    println!("Audio track saved to audiofile.txt");
    Ok(())
}

fn sample_collector<T>(data: &[T], audiotrack: &Arc<Mutex<AudioTrack>>)
where
    T: cpal::Sample + Into<f64>,
{
    let mut audiotrack = audiotrack.lock().unwrap();
    let mut time: f64;
    let mut value: f64;

    for sample in data.iter() {
        let (last_time, _) = *audiotrack.get_track().last().unwrap_or(&(0.0, 0.0));
        time = last_time + 1.0 / audiotrack.bitrate as f64;
        value = (*sample).into() as f64;
        audiotrack.add_sample(time, value);
    }
}
