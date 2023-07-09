use std::{env, process}; //return an iterator of the command line arguments.
use std::fs;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect(); // collect() return all the elements from the command line argument.

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }

}

fn run(config: Config) -> Result<(), Box<dyn Error>> { //dyn -> dynamic
    let content = fs::read_to_string(config.file_path)?;
    println!("with text:\n{content}");

    Ok(())
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
           return panic!("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config {query, file_path})
    }
}

