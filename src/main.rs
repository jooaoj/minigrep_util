use std::{ env, process };

use minigrep_util::{ Config };

fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(&args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    //println!("Searching for '{}'...", &config.query);
    //println!("Found in file '{}'", &config.path);

    // Error from main
    if let Err(e) = minigrep_util::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    use minigrep_util::{ search, search_case_insensitive };

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "RuSt";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, content));
    }
}
