mod cpu;
mod instruction;
mod bus;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let bytes = match fs::read(&args[1]) {
        Ok(t) => t,
        Err(_) => panic!("File not found!")
    };

    processor.
}
