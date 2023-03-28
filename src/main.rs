use std::env;
use grep_clone;

fn main() {
    let args: Vec<String> = env::args().collect();

    let args_result = grep_clone::Args::build(&args);

    if args_result.is_err() {
        grep_clone::handle_parse_error(args_result);
    } else {
        let args = args_result.unwrap();
        let result = grep_clone::run(args.query, args.file_path);
        grep_clone::handle_error(result);
    }
}
