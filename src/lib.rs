use std::error::Error;
use std::fs;

pub struct Args {
    pub query: String,
    pub file_path: String
}

impl Args {
    pub fn build(args: &[String]) -> Result<Args, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        Ok(Args {
            query:  args[1].clone(), 
            file_path: args[2].clone()
        })
    }
}

pub fn run(query: String, file_path: String) -> Result<String, Box<dyn Error>>{
    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("File should have been read");

    println!("With text:\n{contents}");
    Ok(String::from("OK!"))
}

pub fn handle_error(_: Result<String, Box<dyn Error>>) {

}