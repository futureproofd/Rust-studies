use std::error::Error;
use std::fs;

pub struct Config {
    pub filename: String,
    pub query: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments!");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { filename, query })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // '?' (Ch.8) will return the Error of the Result
    let contents = fs::read_to_string(config.filename)?;
    println!("with text: {}", contents);
    // but using () like this is the idiomatic way to indicate that we’re calling
    // run for its side effects only; it doesn’t return a value we need.
    Ok(())
}
