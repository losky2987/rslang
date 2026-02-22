use std::error::Error;
use std::{env, fs};

#[allow(dead_code)]
pub struct Config {
    query: String,
    filepath: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(mut args:impl Iterator<Item=String>,) -> Result<Config, String> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err(String::from("Didn't get a query string")),
        };

        let filepath = match args.next() {
            Some(arg) => arg,
            None => return Err(String::from("Didn't get a file path")),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_ok();

        return Ok(Config {
            query: query,
            filepath: filepath,
            case_sensitive: case_sensitive,
        });
    }
}

pub fn run(config:Config) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(config.filepath.clone())?;

        let res = if config.case_sensitive {
            search(&config.query, &contents)
        } else {
            search_case_insensitive(&config.query, &contents)
        };

        for line in res {
            println!("{line}");
        }
    
        return Ok(());
    }

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    return contents
        .lines()
        .filter(|line| line.contains(query))
        .collect();
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            res.push(line);
        }
    }
    return res;
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
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
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
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}