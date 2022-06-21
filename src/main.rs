use std::env;
use std::fs;

fn main() {
    
    // store arguments from execution call into a vector of strings
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = Config::new(&args);

    // use the second argument as a reference for file read operation
    let content = fs::read_to_string(config.filename)
        .expect("something went wrong reading the file");

    println!("text content:\n{}", content);
}

// by using a struct to store this two values, we convey that
//they are related values and simplify the process of finding them
struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {

        if args.len() < 3 {
            panic!("not enough arguments");
        }
    
        // here we opt on cloning the values received by reference, to avoid having
        //to manage references lifetime - its a trade-off of performance for simplicity
        let query = args[1].clone();
        let filename = args[2].clone();
    
        println!("searching for {}", query);
        println!("in file {}", filename);
    
        Config {query, filename}
    }
}