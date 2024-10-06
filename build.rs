use pkg_config;

fn main() {
    // Use pkg-config to find and link speexdsp
    pkg_config::Config::new()
        .atleast_version("1.2.1")
        .probe("speexdsp")
        .expect("Could not find speexdsp using pkg-config");
}