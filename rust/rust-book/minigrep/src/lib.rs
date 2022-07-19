use std::env;
use std::error::Error;
use std::fs;
use std::ops::RangeBounds;
use std::path::Path;
use std::process;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next();

        let query = match args.next() {
            Some(q) => q,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(f) => f,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Self {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = Path::new(&config.filename);

    if !file.exists() {
        println!("File {:?} does not exist", file);
        process::exit(1);
    }

    let contents = fs::read_to_string(file)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        serach_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn serach_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            result.push(line);
        }
    }

    result
}

// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn test_case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            serach_case_insensitive(query, contents)
        );
    }
}
