use rustfft::{FftPlanner, num_complex::Complex};
use crate::audio::io::AudioTrack;


pub fn lowpass_filter(track: AudioTrack, cutoff_freq: f64) -> Vec<f64> {

    let y = track.get_volume();
    let sample_rate = track.samplerate;

    // Step 1: Perform FFT
    let mut planner = FftPlanner::new();
    let fft_size = y.len();
    let fft = planner.plan_fft_forward(fft_size);
    
    // Convert the signal signal to a vector of complex numbers (real part is the signal, imaginary part is 0)
    let mut signal: Vec<Complex<f64>> = y.iter().map(|&v| Complex::new(v, 0.0)).collect();
    // let mut spectrum: Vec<Complex<f64>> = vec![Complex::new(0.0, 0.0); fft_size];

    fft.process(&mut signal);

    // Step 2: Apply low-pass filter
    let nyquist_freq = sample_rate / 2.0; // Nyquist frequency is half the sampling rate
    let cutoff_bin = (cutoff_freq / nyquist_freq * (fft_size as f64 / 2.0)) as usize;

    for i in cutoff_bin..(fft_size - cutoff_bin) {
        signal[i] = Complex::new(0.0, 0.0); // Zero out frequencies beyond the cutoff
    }

    // Step 3: Perform Inverse FFT
    let ifft = planner.plan_fft_inverse(fft_size);
    ifft.process(&mut signal);

    // Extract the real part of the filtered signal
    signal.iter().map(|c| c.re).collect()
}
