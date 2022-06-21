use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    
    // store arguments from execution call into a vector of strings
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem passing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }

}

fn run (config: Config) -> Result<(), Box<dyn Error>>{

    // here we use the ? for returning the error, else the code goes on
    let content = fs::read_to_string(config.filename)?;

    println!("text content:\n{}", content);

    Ok(())
}

// by using a struct to store this two values, we convey that
//they are related values and simplify the process of finding them
struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("not enough arguments");
        }
    
        // here we opt on cloning the values received by reference, to avoid having
        //to manage references lifetime - its a trade-off of performance for simplicity
        let query = args[1].clone();
        let filename = args[2].clone();
    
        println!("searching for {}", query);
        println!("in file {}", filename);
    
        Ok(Config {query, filename})
    }
}