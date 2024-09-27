use std::env;

fn main() {
    // take in args from the command line
    let args: Vec<String> = env::args().collect();
    // args will print out ['binary path', other arguments...]

    // because args[0] will print out binary path, we need to take args[0] and args[1] for query, filename
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}