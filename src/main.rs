use std::env; //return an iterator of the command line arguments.

fn main() {
    let args: Vec<String> = env::args().collect(); // collect() return all the elements from the command line argument.

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);
}
