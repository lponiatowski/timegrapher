mod ffi {
    use libc::{c_int, c_void, c_float};

    extern "C" {
        // Define the necessary functions from the SpeexDSP library
        pub fn speex_preprocess_state_init(frame_size: c_int, sampling_rate: c_int) -> *mut c_void;
        pub fn speex_preprocess_state_destroy(st: *mut c_void);
        pub fn speex_preprocess_run(st: *mut c_void, x: *mut c_float) -> c_int;
        pub fn speex_preprocess_ctl(st: *mut c_void, request: c_int, ptr: *mut c_void) -> c_int;

        // Add other necessary functions and constants
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum SetControll {
    Denoise = 0,
    Agc = 2,
    Vad = 4,
    AgcLevel = 6,
    Dereverb = 8,
    DereverbLevel = 10,
    DereverbDecay = 12,
    ProbStart = 14,
    ProbContinue = 16,
    NoiseSuppress = 18,
    EchoSuppress = 20,
    EchoSuppressActive = 22,
    EchoState = 24,
    AgcIncrement = 26,
    AgcDecrement = 28,
    AgcMaxGain = 30,
    AgcTarget = 46,
}

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum GetControll {
    Denoise = 1,
    Agc = 3,
    Vad = 5,
    AgcLevel = 7,
    Dereverb = 9,
    DereverbLevel = 11,
    DereverbDecay = 13,
    ProbStart = 15,
    ProbContinue = 17,
    NoiseSuppress = 19,
    EchoSuppress = 21,
    EchoSuppressActive = 23,
    EchoState = 25,
    AgcIncrement = 27,
    AgcDecrement = 29,
    AgcMaxGain = 31,
    AgcLoudness = 33,
    AgcGain = 35,
    PsdSize = 37,
    Psd = 39,
    NoisePsdSize = 41,
    NoisePsd = 43,
    Prob = 45,
    AgcTarget = 47,
}

#[derive(Debug, Clone)]
pub struct Denoiser {
    state: *mut libc::c_void,
}

// Speex is thread unsafe so no Send !!
// Implement Send for Denoiser
// unsafe impl Send for Denoiser {}

impl Denoiser {
    // Initialize the denoiser
    pub fn new(frame_size: i32, sampling_rate: i32) -> Self {
        unsafe {
            let state = ffi::speex_preprocess_state_init(frame_size, sampling_rate);
            Denoiser { state }
        }
    }

    // Run the denoiser
    pub fn process(&self, frame: &mut [f32]) -> bool {
        unsafe {
            ffi::speex_preprocess_run(self.state, frame.as_mut_ptr()) != 0
        }
    }

    pub fn set_ctl(self, request: SetControll, value: i32) -> Self {
        let mut value = value;
        unsafe {
            ffi::speex_preprocess_ctl(self.state, request as i32, &mut value as *mut _ as *mut libc::c_void);
        }
        self
    }

    pub fn get_ctl(&self, request: GetControll, value: i32) -> i32 {
        let mut value = value;
        unsafe {
            ffi::speex_preprocess_ctl(self.state, request as i32, &mut value as *mut _ as *mut libc::c_void)
        }
    }
    
}

impl Drop for Denoiser {
    fn drop(&mut self) {
        unsafe {
            ffi::speex_preprocess_state_destroy(self.state);
        }
    }
}