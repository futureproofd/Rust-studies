use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub filename: String,
    pub query: String,
    pub case_sensitive: bool,
}

impl Config {
    // old impl
    // We needed clone here because we have a slice with String elements in the parameter args, but the new function doesn’t own args
    /*
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments!");
        }
        // to return an instance of Config, we have to close the args so it can have it's own values
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            filename,
            query,
            case_sensitive,
        })
    }
    */

    // take ownership of an iterator as an argument instead of borrowing a slice
    // now we’re passing ownership of the iterator returned from env::args to Config::new directly.
    // Because we’re taking ownership of args and we’ll be mutating args by iterating over it,
    // we can add the mut keyword into the specification of the args parameter to make it mutable.
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments!");
        }
        // we can now manually iterate over our args since it implements an Iterator
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("no query provided!"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("no filename provided!"),
        };
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
    // note the functional approach here opposed to the original used below in the case insensitive version
    /*
    lets us avoid having a mutable intermediate results vector. The functional programming style prefers to
    minimize the amount of mutable state to make code clearer. Removing the mutable state might enable a
    future enhancement to make searching happen in parallel, because we wouldn’t have to manage
    concurrent access to the results vector.
    */
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
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
