use std::env; //return an iterator of the command line arguments.
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect(); // collect() return all the elements from the command line argument.

    let config = Config::new(&args);
    let content = fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("with text:\n{content}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config {query, file_path}
    }
}

