use std::io;

fn main() {
    loop {
        // Separated into two functions, get_user_input and find_fib
        let result = find_fib(get_user_input());

        println!("Fibonacci number: {result}");
    }
}


fn find_fib(n: i64) -> i64 {
    // Find fib follows basic alg
    // require n >= 0
    // a = 0;
    // b = 1;
    // repeat n times {
    //      temp = a;
    //      a = b;
    //      b = temp + b;
    // }
    // return a;
    //
    // Further optimization like memoization would speed up calculations
    //
    // Also it would be better if I stored the numbers in heap instead of on stack
    // Dynamic allocation should eliminate overflow errors
    //
    // Scientific notation once a number threshold is reached,
    // would help prevent integer overflow errors

    if n >= 0 {

        let mut a = 0;
        let mut b = 1;

        println!(); // Space between entered value and results

        for _ in 1..n+1 {
            let temp = a;
            a = b;
            b = temp + b;
            println!("{temp}"); // Print each fib num in sequence
        }
        return a; // Final result. Prone to overflowing, needs to be converted if too high

    } else {
        println!("Please enter a number greater than 0");
        return -1;
    }

}


fn get_user_input() -> i64 {
    // Returns user input used for Nth fib sequence
    // Minimal error handling for system & value errors
    println!("\nEnter Nth fibonacci sequence: ");

    let mut fib_num = String::new();

    io::stdin().read_line(&mut fib_num).expect("System error");

    let fib: i64 = match fib_num.trim().parse() {
        // Evaluating Result enum
        Ok(fib) => fib, // Pass value if Ok
        Err(_) => { // Otherwise Error prints an error message and returns -1
            println!("Please enter a valid integer");
            return -1;
        },
    };
    return fib;
}