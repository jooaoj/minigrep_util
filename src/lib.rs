use std::{ error::Error, fs };

pub struct Config {
    pub query: String,
    pub path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let path = args[2].clone();
    
        Ok(Config{ query, path })
    }   
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        } 
    }

    result
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.path)?; // Instead of panicking with expect, return an error value

    for line in search(&config.query, &content) {
        println!("{line}");
    }

    Ok(())
}
