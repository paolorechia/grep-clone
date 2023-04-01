use std::error::Error;
use std::fs;
use std::env;


pub struct Args {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool
}

impl Args {
    pub fn build(args: &[String]) -> Result<Args, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let mut ignore_case: bool = false;
        if env::var("IGNORE_CASE").is_ok() {
            ignore_case = true;
        }
        if args.len() >= 4 {
            if args[3].contains("-i") || args[3].contains("--ignore-case") {
                ignore_case = true;
            }
        }
        Ok(Args {
            query:  args[1].clone(), 
            file_path: args[2].clone(),
            ignore_case: ignore_case
        })
    }
}

pub fn case_insensitive_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
        result.push(line);
        }
    }
    result
}



pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(args.file_path)?;

    let result: Vec<&str>;

    if args.ignore_case {
        result = case_insensitive_search(&args.query, &contents);
    } else {
        result = search(&args.query, &contents);
    }
    for line in result {
        println!("{line}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query: &str = "RuSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], case_insensitive_search(query, contents));
    }

}