use std::env;

use smash::config::Config;

fn main() {
    let args = env::args().collect();
    Config::new(&args);

    println!("Hello from Hermit!");
}
