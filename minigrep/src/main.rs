use std::{env, process};
use minigrep::Config;

// let's start to learn rust
fn main() {
    let mut args: Vec<String> = env::args().collect();
    // for debug
    if args.len() == 1 {
        args.push(String::from("query"));
        args.push(String::from("poem.txt"));
    }
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

