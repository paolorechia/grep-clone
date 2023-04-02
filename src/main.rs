use std::env;
use std::process;

use grep_clone;

fn main() {
    let args = grep_clone::Args::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let result = grep_clone::run(args);

    if let Err(e) = result {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
