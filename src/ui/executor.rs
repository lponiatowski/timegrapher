use crate::audio::io::AudioStream;
use crate::audio::track::AudioTrack;
use crate::signal::speexdsp;
use std::sync::Arc;
use tokio::{signal, spawn, sync::Mutex, task::JoinHandle};
use crate::signal::utils;

pub struct ExecutorCTL {
    pub rawdata: Arc<Mutex<AudioTrack>>,
    pub data: Arc<Mutex<AudioTrack>>,
    pub duration: f64,
    pub use_denoiser: i32,
    pub noise_supr_level: i32,
    pub use_agc: i32, 
    pub agc_level: i32,
    pub cutoff: f64,
}

pub fn spawn_executor(aust: AudioStream, ctl: ExecutorCTL) -> Option<JoinHandle<()>> {
    // calclulate framesize
    let sampling_rate = aust.samplerate();
    let frame_size: f64 = ctl.duration * sampling_rate;
    let frame_size: i64 = frame_size.round() as i64;

    let handle = spawn(async move {
        loop {
            let mut track = aust.get_track_by_framesize(frame_size).await;
    
            let mut rawdata = ctl.rawdata.lock().await;
            *rawdata = track.clone();

            let frame: Vec<f32> = track.get_volume().iter().map(|&v| v as f32).collect();
            let processed_frame = tokio::task::spawn_blocking(move || {
                // Create the Denoiser and process the frame inside the blocking task
                let speex = speexdsp::Denoiser::new(frame_size as i32, sampling_rate as i32)
                    .set_ctl(speexdsp::SetControll::Denoise, ctl.use_denoiser)
                    .set_ctl(speexdsp::SetControll::NoiseSuppress, ctl.noise_supr_level)
                    .set_ctl(speexdsp::SetControll::Agc, ctl.use_agc)
                    .set_ctl(speexdsp::SetControll::AgcLevel, ctl.agc_level);
                
                let mut frame = frame; // mutable frame
                speex.process(&mut frame);
                frame
            })
            .await
            .unwrap();
    
            track.update_volume(processed_frame);

            track = utils::cutt_off(track, ctl.cutoff);

            let mut data = ctl.data.lock().await;
            *data = track;
        }
    });
    
    Some(handle)
}
