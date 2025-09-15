mod vectors;
mod strings;

pub fn run_all() {
    let decorator_length = 5; 
    println!("Chapter 8: Common Collections");
    
    run_example("Vectors", decorator_length, vectors::run_examples);
    run_example("Strings", decorator_length, strings::run_examples);
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