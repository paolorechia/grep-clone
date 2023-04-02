//! # Grep Clone
//! A mini version of grep that supports searching a file with a substring.
//!
//! It's only implemented here for learning purposes.
//!
//! Project originally specified at: <https://doc.rust-lang.org/book/ch12-00-an-io-project.html>

use std::env;
use std::error::Error;
use std::fs;

/// # Struct that defines required/supported arguments
/// The struct can be used to parse arguments from an iterator.
pub struct Args {
    /// Pattern used as a substring search.
    pub query: String,
    /// Path to the file that should be opened and used for searching.
    pub file_path: String,
    /// Whether the search should be case-sensitive or not.
    pub ignore_case: bool,
}

impl Args {
    /// # Builds arguments from an iterator
    /// Expects argument in the order:
    /// 0. executable_name
    /// 1. query
    /// 2. file_path
    /// 3. ignore_case (optional)
    ///
    /// # Examples
    ///
    /// ```
    /// let iter = vec![
    ///     String::from("grep_clone"),
    ///     String::from("Who"),
    ///     String::from("poem.txt"),
    /// ].into_iter();
    /// let args = grep_clone::Args::build(iter);
    /// ```
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Args, &'static str> {
        args.next();

        let query = match args.next() {
            Some(val) => val,
            None => return Err("Missing query"),
        };

        let file_path = match args.next() {
            Some(val) => val,
            None => return Err("Missing file path"),
        };

        let mut ignore_case: bool = false;
        if env::var("IGNORE_CASE").is_ok() {
            ignore_case = true;
        }

        let next = args.next().unwrap_or(String::from(""));
        if next.contains("-i") || next.contains("--ignore-case") {
            ignore_case = true;
        }

        Ok(Args {
            query,
            file_path,
            ignore_case,
        })
    }
}

/// # Case insensitive search
///
///
///
///
///
pub fn case_insensitive_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
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

        assert_eq!(
            vec!["Rust:", "Trust me."],
            case_insensitive_search(query, contents)
        );
    }
}
