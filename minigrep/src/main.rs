use std::{env, fs};

use minigrep::Config;
use std::process::exit;

fn main() {
    
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        exit(1);
    });
    
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        exit(1);
    }
    
    
}



