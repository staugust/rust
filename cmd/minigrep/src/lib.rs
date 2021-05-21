use std::env;
use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub struct GrepError {
    info: String,
}

impl GrepError {
    fn new(s: &String) -> GrepError {
        let info = s.clone();
        GrepError { info }
    }
}

impl Debug for GrepError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "debug GrepError is {:?}!", self.info)
    }
}

impl Error for GrepError {}

impl Display for GrepError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "display GrepError is {:?}!", self.info)
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("With text:\n{}", contents);
    let vec = search(&config.query, &contents);
    if vec.len() == 0 {
        return Err(Box::from(GrepError::new(&String::from("das"))));
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut vec = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            vec.push(line);
        }
    }
    vec
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "ductss";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
