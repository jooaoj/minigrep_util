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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.path)?; // Instead of panicking with expect, return an error value

    println!("With text:\n```\n{content}\n```");

    Ok(())
}
