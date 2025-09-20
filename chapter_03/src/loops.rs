fn main() {
    let val = false;
    if val {
        loop {
            println!("Hello World!")
        }
    }

    loop_return();
    while_loop();
    while_collection();
    for_loop();
    for_range();
}

fn loop_return() {
    let mut counter = 0;

    let result = loop {
        println!("{counter}");
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Result: {result}");
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn while_collection() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < a.len() { // < NOT <= or index will overflow
        println!("the value is: {}", a[index]);
        index += 1;
    }
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("{element}");
    }
}

fn for_range() {
    for number in (1..6).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}