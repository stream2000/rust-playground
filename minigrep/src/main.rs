use minigrep::Config;
use std::{env, process};

fn main() {
    let mut args: Vec<String> = env::args().collect();

    // for debug
    if args.len() == 1 {
        args.push(String::from("query"));
        args.push(String::from("poem.txt"));
    }

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
