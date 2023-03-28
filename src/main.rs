use std::env;
use grep_clone;

fn main() {
    let args: Vec<String> = env::args().collect();

    let args = grep_clone::parse_args(&args);
    let result = grep_clone::run(args.query, args.file_path);

    grep_clone::handle_error(result);

}
