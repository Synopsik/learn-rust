fn main() {
    println!("Hello World!");
    another_function(5);
    statement_expression();
    let x = five();
    println!("The value of x is: {x}");
}

fn another_function(x: i32) {
    println!("Another Function! {x}");
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}


fn statement_expression() {
    // A statement cannot evaluate to another statement
    // let x = (let y = 3) INCORRECT

    let x = {
        // You can, however, evaluate a statement within a statement evaluation
        let x = 3;
        // and then use that in an expression for the result
        x + 1 // this can't have a ;
    };
    println!("{x}")
}

fn five() -> i32 {
    5 // this technically results in an expression valuing 5
}

fn plus_one(x: i32) -> i32 {
    x + 1 // if we used semicolon(;), nothing would be returned and error
    // x + 1; ERROR this is a statement, the return is expecting an expression implicitly
}
