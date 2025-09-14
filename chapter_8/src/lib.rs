mod vectors;

pub fn run_all() {
    println!("Chapter 8: Common Collections");
    separator_decorator(50);
    
    println!("Vectors:");
    vectors::run_examples();
    separator_decorator(50);
    
    
}

fn separator_decorator(size: usize) 
{
    println!("{}", "~".repeat(size));
}