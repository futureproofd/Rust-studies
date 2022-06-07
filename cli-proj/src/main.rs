use std::env;
use std::process;

use cli_proj::run;
use cli_proj::Config;

fn main() {
    // i.e. cargo run stuff ./file.txt
    // or w/ env variable: CASE_INSENSITIVE=1 cargo run STUFF ./file.txt
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing args: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
