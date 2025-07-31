fn main() {
    null_values();
}

fn null_values() {
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
    
    // We must convert an Option before using is as a valid value
    // For Example,
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    // let sum = x + y; ERROR!!!
    // Must convert y from Option<i8> to i8
    
}
