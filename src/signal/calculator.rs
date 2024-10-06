use crate::audio::track::AudioTrack;
use crate::signal::utils;
use log::info;
use std::collections::HashMap;

pub struct BitCalculator {
    track: AudioTrack,
    x: Vec<f64>,
    y: Vec<f64>,
}

impl BitCalculator {
    pub fn new(track: AudioTrack) -> Self {
        Self {
            track,
            x: Vec::new(),
            y: Vec::new(),
        }
    }

    fn get_mode(numbervec: &Vec<f64>) -> Option<f64> {
        // Create a HashMap to store the frequency of each number
        let mut frequency_map: HashMap<i64, usize> = HashMap::new();
        let precision = 1_000_000; // Set precision to round to 6 decimal places

        // Count the occurrences of each rounded number
        for &num in numbervec {
            let rounded_num = (num * precision as f64).round() as i64; // Round and scale f64 to i64
            *frequency_map.entry(rounded_num).or_insert(0) += 1;
        }

        // Find the number with the highest frequency
        let mut mode = None;
        let mut max_count = 0;

        for (rounded_num, count) in frequency_map {
            if count > max_count {
                max_count = count;
                mode = Some(rounded_num);
            }
        }

        // Convert the mode back to f64 by dividing by the precision
        mode.map(|m| m as f64 / precision as f64)
    }

    fn remove_mode(input: &Vec<f64>) -> Vec<f64>{
        if let Some(mode) = BitCalculator::get_mode(input){
            input.iter().cloned().map(|v| if v > mode { v } else {0.0}).collect()
        } else {
            input.to_owned()
        }
    }
    pub fn run_calculator(&self) -> AudioTrack {
        let samplerate = self.track.get_sample_rate();

        let frame_size: f64 = 0.001 * samplerate;
        let frame_size: usize = frame_size.round() as usize;
        let half_frame = frame_size / 2;

        let (min, max) = utils::get_min_max(&self.track);
        let mean = utils::get_mean(&self.track);

        let track = utils::remove_mean(&self.track);
        let track = utils::abs(&track);

        let threshhold = mean + 0.6 * (max - mean);

        let mut time = track.get_time();
        let mut volu = track.get_volume();
        let track_len = time.len();

        let mut integral_vol: Vec<f64> = Vec::with_capacity(track_len);
        for ind in 0..half_frame {
            integral_vol.push(volu[0..(ind + half_frame)].iter().sum());
        }
        for ind in half_frame..(track_len - half_frame) {
            integral_vol.push(volu[(ind - half_frame)..(ind + half_frame)].iter().sum());
        }
        for ind in (track_len - half_frame)..track_len {
            integral_vol.push(volu[(ind - half_frame)..(track_len - 1)].iter().sum());
        }
        
        integral_vol = BitCalculator::remove_mode(&integral_vol);

        let track = time
            .iter()
            .cloned()
            .zip(integral_vol.iter().cloned())
            .collect::<Vec<(f64, f64)>>();
        AudioTrack::from_rate_track(samplerate, track)
    }
}
