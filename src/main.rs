use std::env; //return an iterator of the command line arguments.
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect(); // collect() return all the elements from the command line argument.

    let (query, file_path) = parse_config(&args);
    let content = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("with text:\n{content}");
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query,file_path)
}
