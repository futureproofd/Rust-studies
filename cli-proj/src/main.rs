use std::env;
use std::process;

use cli_proj::run;
use cli_proj::Config;

fn main() {
    // i.e. cargo run stuff ./file.txt
    // or w/ env variable: CASE_INSENSITIVE=1 cargo run STUFF ./file.txt

    /*
    Old, inefficent way which results in cloning our args into new variables (see lib):
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing args: {}", err);
        process::exit(1);
    });
    */

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing args: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
