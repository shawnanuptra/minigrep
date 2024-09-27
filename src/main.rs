use std::{env, fs};

fn main() {
    // take in args from the command line
    let args: Vec<String> = env::args().collect();
    // args will print out ['binary path', other arguments...]

    let config: Config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // fs (filesystem) reads content file, return as Result<Ok, Err>, where Ok is a String
    let contents = fs::read_to_string(config.filename).expect("File can't be found");

    println!("With text:\n {}", contents)
}

struct Config<'a> {
    query: &'a str,
    filename: &'a str,
}

impl<'a> Config<'a> {
    fn new(args: &[String]) -> Config {
        // because args[0] will print out binary path, we need to take args[0] and args[1] for query, filename
        let query = &args[1];
        let filename = &args[2];

        // return Config instance
        Config { query, filename }
    }
}
