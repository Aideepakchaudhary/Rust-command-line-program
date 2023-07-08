use std::env; //return an iterator of the command line arguments.

fn main() {
    let args: Vec<String> = env::args().collect(); // collect() return all the elements from the command line argument.
    dbg!(args);
}
