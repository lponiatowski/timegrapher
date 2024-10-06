use crate::audio::track::AudioTrack;
use crate::signal::utils;

pub struct BitCalculator{}

impl BitCalculator{
    fn get_mean(track: &AudioTrack) -> f64 {
        let vol = track.get_volume();
        let len = vol.len() as f64;
        vol.iter().sum::<f64>() / len
    }
    
    fn get_min_max(track: &AudioTrack) -> (f64, f64) {
        let vol = track.get_volume();
        let max = vol.iter().max_by(|a, b| a.total_cmp(&b)).unwrap_or(&&0.0);
        let min = vol.iter().max_by(|a, b| b.total_cmp(&a)).unwrap_or(&&0.0);
        (*min, *max)
    }
    
    fn get_mode(track: &AudioTrack) -> f64 {
        f64::default()
    }
    
    pub fn run_calculator(track: &AudioTrack) -> Self{
        let frame_size: f64 = 0.01 * track.get_sample_rate();
        let frame_size: usize = frame_size.round() as usize;

        let (min, max) = BitCalculator::get_min_max(track);
        let mean = BitCalculator::get_mean(track);

        let track = utils::remove_mean(track);

        let threshhold = mean + 0.6*(max - mean);

        let mut track = track.get_track();

         
        
        for (t, v) in track.iter(){
            let mut current_max = 0.0;
            let mut max_time = 0.0;
            if *v >= threshhold {
                current_max = *v
            }
        }

        Self {  }
    }
}
