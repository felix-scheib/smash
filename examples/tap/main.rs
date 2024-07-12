use std::env::args;

#[cfg(target_os = "hermit")]
use hermit as _;
use smash::config::Config;

fn main() {
    let args = args().collect();
    Config::new(&args);
}
