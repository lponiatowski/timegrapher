use crate::ui::defs::*;

#[derive(Debug, Clone)]
pub struct AudioSettings {
    is_open: bool,
    pub sample_size: Setting<f64>,
    pub use_denoiser: Setting<bool>,
    pub noise_supr_level: Setting<i32>,
    pub use_agc: Setting<bool>,
    pub agc_level: Setting<i32>,
    pub cutoff: Setting<f64>,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            is_open: false,
            sample_size: Setting::new(5.0),
            use_denoiser: Setting::new(true),
            noise_supr_level: Setting::new(8000),
            use_agc: Setting::new(true),
            agc_level: Setting::new(16000),
            cutoff: Setting::new(-60.0),
        }
    }
}

impl AppSettingCollection for AudioSettings {
    fn is_open(&self) -> &bool {
        &self.is_open
    }

    fn is_open_mut(&mut self) -> &mut bool {
        &mut self.is_open
    }
}

#[derive(Debug, Clone)]
pub struct PlotSettings {
    is_open: bool,
    pub y_limit: Setting<f64>,
}

impl Default for PlotSettings {
    fn default() -> Self {
        Self {
            is_open: false,
            y_limit: Setting::new(0.01),
        }
    }
}

impl AppSettingCollection for PlotSettings {
    fn is_open(&self) -> &bool {
        &self.is_open
    }

    fn is_open_mut(&mut self) -> &mut bool {
        &mut self.is_open
    }
}

#[derive(Debug, Clone)]
pub struct NewError {
    is_error: bool,
    msg: String,
}

impl Default for NewError {
    fn default() -> Self {
        Self {
            is_error: false,
            msg: String::new(),
        }
    }
}

impl ProcessError for NewError {
    fn is_error(&self) -> &bool {
        &self.is_error
    }

    fn is_error_mut(&mut self) -> &mut bool {
        &mut self.is_error
    }

    fn get_message(&self) -> &String {
        &self.msg
    }

    fn get_message_mut(&mut self) -> &mut String {
        &mut self.msg
    }
}
