use crate::audio::io::AudioTrack;



pub fn apply_gain(track: AudioTrack, gain: f64) -> AudioTrack {
    let rate: f64 = track.get_sample_rate();
    let time: Vec<f64> = track.get_time().to_owned();
    let vol: Vec<f64> = track.get_volume().to_owned();

    let vol = vol.iter().map(|&v| gain*v).collect::<Vec<f64>>();
    let track = time.iter().cloned().zip(vol.iter().cloned()).collect::<Vec<(f64, f64)>>();

    AudioTrack::from_rate_track(rate, track)
}

pub fn remove_mean(track: AudioTrack) -> AudioTrack {
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
        .map(|v| {
            if v.abs() < mean{
                0.0
            } else {
                v
            }
        })
        .collect::<Vec<f64>>();

    let track = time
        .iter()
        .cloned()
        .zip(vol.iter().cloned())
        .collect::<Vec<(f64, f64)>>();
    AudioTrack::from_rate_track(rate, track)
}

pub fn cutt_off(track: AudioTrack, cutoff: f64) -> AudioTrack {
    let rate = track.get_sample_rate();
    let time = track.get_time().to_owned();
    let vol = track.get_volume().to_owned();

    let vol = vol
        .iter()
        .map(|&v| {
            if v.abs() < cutoff {
                0.0
            } else {
                v
            }
        })
        .collect::<Vec<f64>>();

    let track = time
        .iter()
        .cloned()
        .zip(vol.iter().cloned())
        .collect::<Vec<(f64, f64)>>();
    AudioTrack::from_rate_track(rate, track)
}
