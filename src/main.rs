use std::{ env, fs };



fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(&args);
    let query = &args[1];
    let path = &args[2];
    let content = fs::read_to_string(path).expect("File does not exist");

    println!("Searching for {query}...");
    println!("In file:\n´´´{path}\n{content}\n´´´");
}
