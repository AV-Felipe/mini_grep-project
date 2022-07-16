use std::env;
use std::process;

use minigrep::Config;


fn main() {
    
    // store arguments from execution call into a vector of strings
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem passing arguments: {}", err);
        process::exit(1);
    });

    // we prefixed the run function (which is a public function in our api/lib.rs) with our crate name
    // for the error handling we are using an "if let" expression:
    //if (let pattern_to_be_matched) = (value_to_be_compared_against the_pattern) {code_to_be_run_if_match}
    // view: https://doc.rust-lang.org/reference/expressions/if-expr.html
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }

}
