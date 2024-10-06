use std::convert::Into;

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

    pub fn get_track(&self) -> Vec<(f64, f64)>{
        let t = self.get_time();
        let v = self.get_volume();
        t.iter().cloned().zip(v.iter().cloned()).collect::<Vec<(f64,f64)>>()
    }

    pub fn from_rate_track(samplerate: f64, track: Vec<(f64, f64)>) -> Self {
        AudioTrack{
            samplerate,
            track
        }
    }

    pub fn update_time<T>(&mut self, time: Vec<T>) -> Self
    where T: Into<f64> + Copy,
    {
        let sr = self.get_sample_rate();
        let volume = self.get_volume();
        let track: Vec<(f64,f64)> = time.iter().map(|&t| t.into()).zip(volume.iter().cloned()).collect();
        AudioTrack::from_rate_track(sr, track)
    }

    pub fn update_volume<T>(&mut self, volume: Vec<T>) -> Self
    where T: Into<f64> + Copy,
    {
        let sr = self.get_sample_rate();
        let time = self.get_time();
        let track: Vec<(f64,f64)> = time.iter().cloned().zip(volume.iter().map(|&v| v.into())).collect();
        AudioTrack::from_rate_track(sr, track)
    }
}