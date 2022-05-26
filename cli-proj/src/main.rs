use std::env;
use std::process;

use cli_proj::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing args: {}", err);
        process::exit(1);
    });

    if let Err(e) = cli_proj::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
