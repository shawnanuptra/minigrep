use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // fs (filesystem) reads content file, return as Result<Ok, Err>, where Ok is a String
    let contents = fs::read_to_string(config.filename)?;

    // loop over the Vec<&str> from search(), and print to console
    for line in search(&config.query, &contents) {
        println!("{}", line)
    }

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

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }
}
