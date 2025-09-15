mod vectors;
mod strings;
mod hash_maps;

pub fn run_all() {
    let decorator_length = 3; 
    separator_decorator(decorator_length);
    println!("Chapter 8: Common Collections");
    
    run_example("Vectors", decorator_length, vectors::run_examples);
    run_example("Strings", decorator_length, strings::run_examples);
    run_example("Hash Maps", decorator_length, hash_maps::run_examples);
}

fn run_example<F>(title: &str, length: usize, func: F) where F: FnOnce()
{
    separator_decorator(length);
    println!("{}", title);
    separator_decorator(length);
    func();
}
fn separator_decorator(size: usize) 
{
    let chunk_size = 25;
    println!("{}", "~".repeat(size * chunk_size));
}