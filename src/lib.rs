use std::error::Error;
use std::fs;


pub fn parse_args(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}

pub fn run(query: &str, file_path: &str) -> Result<String, Box<dyn Error>>{
    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("File should have been read");

    println!("With text:\n{contents}");
    Ok(String::from("OK!"))
}

pub fn handle_error(_: Result<String, Box<dyn Error>>) {

}