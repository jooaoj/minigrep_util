use std::{ env::{self}, error::Error, fs };

pub struct Config {
    pub query: String,
    pub path: String,
    pub ignore_case: bool,
}

impl Config {
    // generic type with trait bounds "Iterator" and return String
    pub fn build(mut args: impl Iterator<Item = String>,) -> Result<Config, &'static str> {
        args.next(); // skip over the path of executable

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string!"),
        };

        let path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path!"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();
    
        Ok(Config{ query, path, ignore_case })
    }   
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content.lines().filter(| line | line.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase(); // not 100 % accurate with all Unicode!!

    content.lines().filter(| line | line.to_lowercase().contains(&query)).collect()
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.path)?; // Instead of panicking with expect, return an error value

    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };

    for line in result {
        println!("{line}");
    }
    
    Ok(())
}
