use std::fs;
use std::error::Error;
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {

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


pub fn run (config: Config) -> Result<(), Box<dyn Error>>{

    // here we use the ? for returning the error, else the code goes on
    let content = fs::read_to_string(config.filename)?;

    println!("text content:\n{}", content);

    Ok(())
}
