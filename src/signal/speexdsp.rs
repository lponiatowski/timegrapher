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
    // Denoiser state
    Denoise = 0,
    //  Automatic Gain Control state
    Agc = 2,
    //  Voice Activity Detection state
    Vad = 4,
    //  Automatic Gain Control level
    AgcLevel = 6,
    //  Reverberation removal state
    Dereverb = 8,
    //  Reverberation removal (Does not work)
    DereverbLevel = 10,
    //  Reverberation removal decay (Does not work)
    DereverbDecay = 12,
    // Set probability required for the VAD to go from silence to voice
    ProbStart = 14,
    // Set probability required for the VAD to stay in the voice state (integer percent)
    ProbContinue = 16,
    // Set maximum attenuation of the noise in dB (negative number)
    NoiseSuppress = 18,
    // Set maximum attenuation of the residual echo in dB (negative number)
    EchoSuppress = 20,
    // Set maximum attenuation of the residual echo in dB when near end is active (negative number)
    EchoSuppressActive = 22,
    // Set the corresponding echo canceller state so that residual echo suppression can be performed (NULL for no residual echo suppression)
    EchoState = 24,
    // Set maximal gain increase in dB/second (int32)
    AgcIncrement = 26,
    // Set maximal gain decrease in dB/second (int32)
    AgcDecrement = 28,
    // Set maximal gain in dB (int32)
    AgcMaxGain = 30,
    // Set preprocessor Automatic Gain Control level (int32)
    AgcTarget = 47,
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
    // Get loudness
    AgcLoudness = 33,
    // Get current gain (int32 percent)
    AgcGain = 35,
    // Get spectrum size for power spectrum (int32)
    PsdSize = 37,
    // Get power spectrum (int32[] of squared values)
    Psd = 39,
    // Get spectrum size for noise estimate (int32)
    NoisePsdSize = 41,
    // Get noise estimate (int32[] of squared values)
    NoisePsd = 43,
    // Get speech probability in last frame (int32).
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