use std::process;
use std::env;


fn main() {
    // Get args from user input
    let args = env::args().collect::<Vec<String>>();
    
    // Create config
    let config = minigrep::Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    
    // Print args for user
    println!("Search for word: {}\nIn the file: {}\n", 
             config.query, config.file_path);
    
    // Run program using config from args
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
    
}






