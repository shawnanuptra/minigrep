use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // fs (filesystem) reads content file, return as Result<Ok, Err>, where Ok is a String
    let contents = fs::read_to_string(config.filename)?;
    println!("With text:\n {}", contents);

    Ok(())
}

pub struct Config<'a> {
    pub query: &'a str,
    pub filename: &'a str,
}

impl<'a> Config<'a> {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        // handle error if args.len() < 3
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        // because args[0] will print out binary path, we need to take args[0] and args[1] for query, filename
        let query = &args[1];
        let filename = &args[2];

        // return Config instance
        Ok(Config { query, filename })
    }
}
