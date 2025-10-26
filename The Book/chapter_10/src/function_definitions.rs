pub fn run_function_definitions() {
    function_definitions();
}

// --------------------------------------------------------------------------------------------- //
//                                  Function Definitions                                         //
// --------------------------------------------------------------------------------------------- //

fn function_definitions() {
    let number_list = vec![34, 50, 25, 100, 65];
    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let result = largest(&char_list);
    println!("The largest character is {result}");
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > &largest {
            largest = item;
        }
    }
    largest
}