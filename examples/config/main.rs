use std::{env, fs};

#[cfg(target_os = "hermit")]
use hermit as _;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Args: {:#?}", args);
    println!("Reding config!");


    let file_path = "/root/writer.config.yml";
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");

}
