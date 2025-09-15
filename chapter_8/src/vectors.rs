pub fn run_examples() {
    values_with_vectors();
    reading_elements_of_vectors();
    iterating_over_vector();
    enums_store_multiple_types();
    println!();
}

fn values_with_vectors() {
    // Create a new empty vector
    let _v: Vec<i32> = Vec::new();
    
    // Vector macro vec! will create a new vector that holds the given values
    let _v = vec![1, 2, 3];
    
    // Update a vector using the push method
    // Since we are pushing i32 values after, the compiler can infer the type
    // Otherwise, we would need to explicitly set the type
    let mut v = Vec::new();
    
    // Compiler infers types thanks to values in these method calls
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
}

fn reading_elements_of_vectors() {
    let v = vec![1, 2, 3, 4, 5];
    println!("{:?}", v);
    
    // Indexing to read an element
    let third: &i32 = &v[2];
    // Useful if we want the program to crash when an index is nonexistent
    println!("The third element is {third}");
    
    // Get Method to read an element
    let fourth: Option<&i32> = v.get(3);
    // This is useful if accessing an element beyond normal range may sometimes happen
    match fourth {
        Some(fourth) => println!("The fourth element is {fourth}"),
        None => println!("There is no third element!")
    }
    
    // Error Scope
    {
        // We can't have a mutable and immutable reference in the same scope
        let mut v = vec![1, 2, 3, 4, 5];
        
        let _first = &v[0];
        
        v.push(6);
        /* 
         * If the memory location for v is full before pushing, 
         * all the data will be copied to a new location, so additional data can be pushed.
         * An error will occur if our immutable var `first` tries to access `v` at the old location
         * Because of this common error, Rust doesn't allow it to ever happen to begin with.
         */
        
        // Remove the following comment to examine this common error
        // println!("The first element is: {first}");
    }
}

fn iterating_over_vector() {
    { 
        // Using for loog to get immutable references to each element in a vector
        let v = vec![100, 32, 57];
        println!("\nIterating over immutable references:");
        for i in &v {
            println!("{i}");
        }
    }
    {
        // We can also iterate over mutable references to each element,
        // making changes to all the elements.
        let mut v = vec![100, 32, 57];
        println!("\nIterating operations over mutable references:");
        println!("Before: {:?}", v);
        for i in &mut v {
            *i += 50; // Dereference Operator * used to get the value of i, before we can use +=
        }
        println!("After adding 50: {:?}", v);
    }
}

fn enums_store_multiple_types() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("\nSpreadsheet Cells Enum:");
    row.iter().for_each(|x| println!("{:?}", x));

}