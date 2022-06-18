use std::env;
use std::fs;

fn main() {
    
    // store arguments from execution call into a vector of strings
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let query = &args[1];
    let filename = &args[2];

    println!("searching for {}", query);
    println!("in file {}", filename);

    // use the second argument as a reference for file read operation
    let content = fs::read_to_string(filename)
        .expect("something went wrong reading the file");

    println!("text content:\n{}", content);
}
