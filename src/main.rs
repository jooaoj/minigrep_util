use std::{ env, process };

use minigrep_util::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(&args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for '{}'...", &config.query);
    println!("Found in file '{}'", &config.path);

    // Error from main
    if let Err(e) = minigrep_util::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
