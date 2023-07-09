use std::{env, process}; //return an iterator of the command line arguments.
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect(); // collect() return all the elements from the command line argument.

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }

}


