use std::process;
use std::env;
mod lib;
use lib::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = lib::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
