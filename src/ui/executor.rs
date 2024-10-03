use tokio::{spawn, task::JoinHandle, sync::Mutex};
use std::sync::Arc;
use crate::audio::io::AudioStream;
use crate::audio::track::AudioTrack;
use crate::signal::utils;
use crate::signal::speexdsp;

pub struct ExecutorCTL{
    pub rawdata: Arc<Mutex<AudioTrack>>,
    pub data: Arc<Mutex<AudioTrack>>,
    pub duration: f64,
    pub gain: f64,
    pub cutoff: f64,
    pub romeve_mean: bool
}

pub fn spawn_executor(aust: AudioStream, ctl: ExecutorCTL) -> Option<JoinHandle<()>> {


    
    println!("Sampling initiated");
    // calclulate framesize 
    let sampling_rate = aust.samplerate();
    let frame_size: f64 = ctl.duration / sampling_rate;
    let frame_size: i64 = frame_size.round() as i64;

    Some(spawn(async move {
        let speex = speexdsp::Denoiser::new(frame_size  as i32, sampling_rate as i32)
        .set_ctl(speexdsp::ControlSet::SetDenoise, 1)
        .set_ctl(speexdsp::ControlSet::SetAgc, 1);

            loop {
                let mut track = aust.get_track_by_framesize(frame_size).await;

                let mut rawdata = ctl.rawdata.lock().await;
                *rawdata = track.clone();

                // track = utils::apply_gain(track, ctl.gain);
                // if ctl.romeve_mean {
                //     track = utils::remove_mean(track);
                // }
                // // track = utils::apply_diff(track);
                // // track = utils::sliding_mean(track, 100);
                // track = utils::sliding_max(track, 300);
                // track = utils::cutt_off( track, ctl.cutoff);
                // track = utils::abs(track);
                let mut frame: Vec<f32> = track.get_volume().iter().map(|&v| v as f32).collect();
                speex.process(&mut frame);

                let mut data = ctl.data.lock().await;
                *data = track;

            }
            
        }))
}