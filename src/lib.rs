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
    
        Ok(Config {query, filename})
    }
}

// this function returns a result tha may be an empty tuple, on ok, or a struct that implements the Error trait
// because we can have objects of different types that implement the same trait, we must pass it as dyn
// also, because we cannot know its size on compile time, we must set it in a Box<T> pointer (it will be stored on the heap)
pub fn run (config: Config) -> Result<(), Box<dyn Error>>{

    // here we use the ? for returning the error, else the code goes on
    let content = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &content) {
        println!("{}", line);
    }

    Ok(())
}

// lifetime here is telling that the string slice we are returning is referencing
//the content parameter, and must exist for as long as that parameter data exists
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query,contents));
    }
}