use crate::audio::track::AudioTrack;

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
        Self {  }
    }
}
