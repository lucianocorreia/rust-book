#![allow(unused)]

use std::{
    env::{self, args, Args},
    error::Error,
    fs::read_to_string,
    process::exit,
};

use minigrep::Config;

fn main() {
    // let args: Vec<String> = args().collect();
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        exit(1);
    });

    println!("-> Searching for {}", config.query);
    println!("-> In file {}\n", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        exit(1);
    }
}
