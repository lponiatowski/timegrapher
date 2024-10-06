use crate::audio::track::AudioTrack;
use log::info;

pub fn apply_gain(track: &AudioTrack, gain: f64) -> AudioTrack {
    let rate: f64 = track.get_sample_rate();
    let time: Vec<f64> = track.get_time().to_owned();
    let vol: Vec<f64> = track.get_volume().to_owned();

    let vol = vol.iter().map(|&v| gain * v).collect::<Vec<f64>>();
    let track = time
        .iter()
        .cloned()
        .zip(vol.iter().cloned())
        .collect::<Vec<(f64, f64)>>();

    AudioTrack::from_rate_track(rate, track)
}

pub fn remove_mean(track: &AudioTrack) -> AudioTrack {
    let rate: f64 = track.get_sample_rate();
    let time: Vec<f64> = track.get_time().to_owned();
    let vol: Vec<f64> = track.get_volume().to_owned();

    // positive mean
    let mean = vol
        .iter()
        .cloned()
        .filter(|&vol| vol >= 0.0)
        .collect::<Vec<f64>>();
    let mean_len = mean.len();
    let mean_sum: f64 = mean.iter().sum();
    let mean_pos: f64 = if mean_len > 0 {
        mean_sum / mean_len as f64
    } else {
        0.0
    };

    // negative mean
    let mean = vol
        .iter()
        .cloned()
        .filter(|&vol| vol < 0.0)
        .collect::<Vec<f64>>();
    let mean_len = mean.len();
    let mean_sum: f64 = mean.iter().sum();
    let mean_neg: f64 = if mean_len > 0 {
        mean_sum.abs() / mean_len as f64
    } else {
        0.0
    };

    // max mean
    let mean = mean_pos.max(mean_neg);

    let vol: Vec<f64> = vol
        .iter()
        .cloned()
        .map(|v| if v.abs() < mean { 0.0 } else { v })
        .collect::<Vec<f64>>();

    let track = time
        .iter()
        .cloned()
        .zip(vol.iter().cloned())
        .collect::<Vec<(f64, f64)>>();
    AudioTrack::from_rate_track(rate, track)
}

pub fn cutt_off(track: &AudioTrack, cutoff: f64) -> AudioTrack {
    let rate = track.get_sample_rate();
    let time = track.get_time().to_owned();
    let vol = track.get_volume().to_owned();

    let cutoff: f64 = 10.0_f64.powf(cutoff / 20.0);

    let vol = vol
        .iter()
        .map(|&v| if v.abs() < cutoff { 0.0 } else { v })
        .collect::<Vec<f64>>();

    let track = time
        .iter()
        .cloned()
        .zip(vol.iter().cloned())
        .collect::<Vec<(f64, f64)>>();
    AudioTrack::from_rate_track(rate, track)
}

pub fn sliding_max(track: &AudioTrack, window: usize) -> AudioTrack {
    let rate = track.get_sample_rate();
    let time = track.get_time().to_owned();
    let mut vol = track.get_volume().to_owned();

    let len = vol.len();
    let lefover = len % window;

    for i in (0..(len - lefover)).step_by(window) {
        let max = vol[i..(i + window)]
            .iter()
            .cloned()
            .max_by(|a, b| a.total_cmp(b))
            .unwrap_or(0.0);

        for j in 0..window {
            vol[i + j] = max;
        }
    }

    let track = time
        .iter()
        .cloned()
        .zip(vol.iter().cloned())
        .collect::<Vec<(f64, f64)>>();
    AudioTrack::from_rate_track(rate, track)
}

pub fn sliding_mean(track: &AudioTrack, window: usize) -> AudioTrack {
    let rate = track.get_sample_rate();
    let time = track.get_time().to_owned();
    let mut vol = track.get_volume().to_owned();

    let len = vol.len();
    let lefover = len % window;

    for i in (0..(len - lefover)).step_by(window) {

        let mut mean = vol[i..(i + window)]
            .iter()
            .cloned()
            .sum();
        mean = mean / window as f64;
        for j in 0..window {
            vol[i + j] = mean;
        }
    }

    let track = time
        .iter()
        .cloned()
        .zip(vol.iter().cloned())
        .collect::<Vec<(f64, f64)>>();
    AudioTrack::from_rate_track(rate, track)
}

pub fn apply_diff(track: &AudioTrack) -> AudioTrack {
    let rate = track.get_sample_rate();
    let time = track.get_time().to_owned();
    let vol = track.get_volume().to_owned();

    let len = vol.len();

    let mut diff = Vec::with_capacity(len);

    // Calculate the differences
    for i in 0..len - 1 {
        diff.push(vol[i + 1] - vol[i]);
    }

    diff.insert(0, f64::default()); // Push zero for the first element

    let track = time
        .iter()
        .cloned()
        .zip(diff.iter().cloned())
        .collect::<Vec<(f64, f64)>>();
    AudioTrack::from_rate_track(rate, track)
}

pub fn abs(track: &AudioTrack) -> AudioTrack {
    let rate = track.get_sample_rate();
    let time = track.get_time().to_owned();
    let mut vol = track.get_volume().to_owned();

    vol = vol.iter().map(|&v| v.abs()).collect::<Vec<f64>>();
    let track = time
        .iter()
        .cloned()
        .zip(vol.iter().cloned())
        .collect::<Vec<(f64, f64)>>();
    AudioTrack::from_rate_track(rate, track)
}