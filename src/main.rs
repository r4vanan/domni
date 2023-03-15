use std::env;
use std::io::{self, BufRead};

pub mod engine;
use engine::crt;
use engine::alien_vault;

fn main() {
    let mut domain = String::new();

    // Check if command line arguments were provided
    if let Some(arg) = env::args().nth(1) {
        domain = arg;
    } else {
        // Read from standard input if no command line arguments were provided
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        handle.read_line(&mut domain).unwrap();
        domain = domain.trim().to_owned();
    }
    alien_vault::alien(&domain).expect("domain not found!");
    crt::get_main(&domain).expect("domain not found!");
}
