use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub filename: String,
    pub query: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments!");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            filename,
            query,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // '?' (Ch.8) will return the Error of the Result
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line)
    }
    // but using () like this is the idiomatic way to indicate that we’re calling
    // run for its side effects only; it doesn’t return a value we need.
    Ok(())
}

// tell Rust that the data returned by the search function will live
// as long as the data passed into the search function in the contents argument.
/*
Rust can’t possibly know which of the two arguments we need, so we need to tell it.
 Because contents is the argument that contains all of our text and we want to return
  the parts of that text that match, we know contents is the argument that should be connected
   to the return value using the lifetime syntax.
*/
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        // pass our Heap query String as a 'slice' as specified by contains method
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "RUST";
        let contents = "\
Rust:
safe, fast, productive.
trust me.";

        assert_eq!(
            vec!["Rust:", "trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
