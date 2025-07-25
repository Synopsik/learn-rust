fn main() {
    let number = 8;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    elif_branches();
    if_in_let();
}

fn elif_branches() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 2, 3, or 4");
    }
}

// condition in a let statement
fn if_in_let() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}")
}