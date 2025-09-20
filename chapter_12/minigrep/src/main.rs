use std::env;
use std::process;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    
    println!("Search for word: {}\nIn the file: {}\n", 
             config.query, config.file_path)
    
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 { return Err("Not enough arguments"); }
        
        let query = args[1].clone();
        let file_path = args[2].clone();
        
        Ok(Config { query, file_path })
    }
}
