fn main() {
    // Specify the path to the SpeexDSP library
    println!("cargo:rustc-link-search=native=/usr/local/Cellar/speexdsp/1.2.1/lib");
    println!("cargo:rustc-link-lib=dylib=speexdsp");
}