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
pub enum ControlConst {
    SetDenoise = 0,
    GetDenoise = 1,
    SetAgc = 2,
    GetAgc = 3,
    SetVad = 4,
    GetVad = 5,
    SetAgcLevel = 6,
    GetAgcLevel = 7,
    SetDereverb = 8,
    GetDereverb = 9,
    SetDereverbLevel = 10,
    GetDereverbLevel = 11,
    SetDereverbDecay = 12,
    GetDereverbDecay = 13,
    SetProbStart = 14,
    GetProbStart = 15,
    SetProbContinue = 16,
    GetProbContinue = 17,
    SetNoiseSuppress = 18,
    GetNoiseSuppress = 19,
    SetEchoSuppress = 20,
    GetEchoSuppress = 21,
    SetEchoSuppressActive = 22,
    GetEchoSuppressActive = 23,
    SetEchoState = 24,
    GetEchoState = 25,
    SetAgcIncrement = 26,
    GetAgcIncrement = 27,
    SetAgcDecrement = 28,
    GetAgcDecrement = 29,
    SetAgcMaxGain = 30,
    GetAgcMaxGain = 31,
    GetAgcLoudness = 33,
    GetAgcGain = 35,
    GetPsdSize = 37,
    GetPsd = 39,
    GetNoisePsdSize = 41,
    GetNoisePsd = 43,
    GetProb = 45,
    SetAgcTarget = 46,
    GetAgcTarget = 47,
}


pub struct Denoiser {
    state: *mut libc::c_void,
}

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

    pub fn set_ctl(&self, request: ControlConst, value: i32) -> i32 {
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