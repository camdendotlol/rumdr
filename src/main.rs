use std::{ env, process };
use rumdr::config;

fn main() {
    let config = config::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = rumdr::run(config) {
        eprintln!("Error: {}", e);
        process::exit(1);
    };
}
