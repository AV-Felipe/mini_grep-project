use std::env;
use std::process;

use minigrep::Config;


fn main() {
    
    // store arguments from execution call into a vector of strings
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem passing arguments: {}", err);
        process::exit(1);
    });

    // we prefixed the run function (which is a public function in our api/lib.rs) with our crate name
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }

}
