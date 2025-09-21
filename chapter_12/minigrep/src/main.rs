use std::process;
use std::env;


fn main() {
    // Get args from user input
    let args = env::args().collect::<Vec<String>>();
    
    // Create config
    let config = minigrep::Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    
    // Run program using config from args
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}






