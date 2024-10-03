use crate::audio::io::AudioStream;
use crate::audio::track::AudioTrack;
use crate::signal::speexdsp;
use crate::signal::utils;
use std::sync::Arc;
use tokio::{spawn, sync::Mutex, task::JoinHandle};

pub struct ExecutorCTL {
    pub rawdata: Arc<Mutex<AudioTrack>>,
    pub data: Arc<Mutex<AudioTrack>>,
    pub duration: f64,
    pub gain: f64,
    pub cutoff: f64,
    pub romeve_mean: bool,
}

pub fn spawn_executor(aust: AudioStream, ctl: ExecutorCTL) -> Option<JoinHandle<()>> {
    println!("Sampling initiated");
    // calclulate framesize
    let sampling_rate = aust.samplerate();
    let frame_size: f64 = ctl.duration * sampling_rate;
    let frame_size: i64 = frame_size.round() as i64;

    let handle = spawn(async move {
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
            let frame: Vec<f32> = track.get_volume().iter().map(|&v| v as f32).collect();
            let processed_frame = tokio::task::spawn_blocking(move || {
                // Create the Denoiser and process the frame inside the blocking task
                let speex = speexdsp::Denoiser::new(frame_size as i32, sampling_rate as i32)
                    .set_ctl(speexdsp::SetControll::Denoise, 1)
                    .set_ctl(speexdsp::SetControll::NoiseSuppress, 16000)
                    .set_ctl(speexdsp::SetControll::Agc, 1)
                    .set_ctl(speexdsp::SetControll::AgcLevel, 8000);
                
                let mut frame = frame; // mutable frame
                speex.process(&mut frame);
                frame
            })
            .await
            .unwrap();
    
            track.update_volume(processed_frame);

            let mut data = ctl.data.lock().await;
            *data = track;
        }
    });
    
    Some(handle)
}
