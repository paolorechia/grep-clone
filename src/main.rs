use std::env;
use std::process;

use grep_clone;

fn main() {
    let args: Vec<String> = env::args().collect();

    let args = grep_clone::Args::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let result = grep_clone::run(args);

    if let Err(e) = result {
        println!("Application error: {e}");
        process::exit(1);
    }
}
