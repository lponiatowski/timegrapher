fn main() {
    // Specify the path to the SpeexDSP library /Users/luka/system/homebrew/Cellar/speexdsp/1.2.1/lib
    println!("cargo:rustc-link-search=native=/Users/luka/system/homebrew/Cellar/speexdsp/1.2.1/lib");
    println!("cargo:rustc-link-lib=dylib=speexdsp");
    println!("cargo:rustc-env=LD_LIBRARY_PATH=/Users/luka/system/homebrew/Cellar/speexdsp/1.2.1/lib");
}
