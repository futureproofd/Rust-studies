use std::fs::File;
use std::io::{ErrorKind, Read};

fn main() {
    let f = File::open("hey.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("problem creating the file {:?}", e),
            },
            other_error => {
                panic!("problem opening the file {:?}", other_error);
            }
        },
    };

    read_username_from_file();
}

pub struct Guess {
    value: i32,
}

// create a contract for a new Guess which enforces value range, or else panics.
/*
    A function that has a parameter or returns only numbers between 1 and 100 could then declare
 in its signature that it takes or returns a Guess rather than an i32 and wouldn’t need to
 do any additional checks in its body.
*/
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Value must be between 1 and 100");
        }
        Guess { value }
    }
    // public getter. Avoid setting value directly
    pub fn value(&self) -> i32 {
        self.value
    }
}

fn read_username_from_file() -> Result<String, std::io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// Same function with '?' shortcut.
// similar to match expression, for a function that returns a Result, The '?' Will return an error type, or else if it succeeds, returns Ok
// the difference is that the returned error will propagate as whatever error type was thrown from the statement.
fn simplified_username_from_file() -> Result<String, std::io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn simplified_further_username_from_file() -> Result<String, std::io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// standard library compact version:
fn std_read_username_from_file() -> Result<String, std::io::Error> {
    std::fs::read_to_string("hello.txt")
}

// '?' shortcut also works with Option
// if lines() iterator followed by next() doesn't return a character, it will return None, otherwise Some
// The ? allows us to do this
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
