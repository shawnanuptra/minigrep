use minigrep::Config;
use std::{env, process};

fn main() {
    // take in args from the command line
    let args: Vec<String> = env::args().collect();
    // args will print out ['binary path', other arguments...]

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing args: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error {}", e);
        process::exit(1);
    }
}
