use std::env;
use grep_clone;

fn main() {
    let args: Vec<String> = env::args().collect();

    let args = grep_clone::parse_args(&args);
    let query = args.0;
    let file_path = args.1;

    let result = grep_clone::run(query, file_path);

    grep_clone::handle_error(result);

}
