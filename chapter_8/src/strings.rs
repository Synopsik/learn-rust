pub fn run_examples() {
    creating_new_string();
    updating_a_string();
    concatenation();    
}

fn creating_new_string() {
    println!("Creating A String");
    let mut s = String::new();
    
    let data = "initial contents";
    
    let s = data.to_string();
    println!("s = '{0}'", &s);
    
    // The method also works on a literal directly:
    let s = "initial contents".to_string();
}

fn updating_a_string() {
    println!("\nUpdating a String");
    let mut s = String::from("foo");
    println!("s = '{0}'", &s);
    s.push_str("bar");
    println!("s = '{0}'", &s);
    
    // Using our string slice after appending it to a String
    let mut s1 = String::from("foo");
    let s2 = "bar"; // Give s2 ownership of string slice "bar"
    s1.push_str(s2);
    println!("s2 is {s2}");
    
    // Adding a single char to a String using the push method
    let mut s = String::from("lo");
    s.push('l');
    println!("s = '{0}'", &s);
}

fn concatenation() {
    println!("\nConcatenation");
    // Concatenation using the + operator
    let s1 = String::from("Hello, ");
    println!("s1 = '{0}'", s1);
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // Ownership of s1 transfers to s3, only take ref of s2
    println!(
        "s2 = '{0}'\ns3 = '{1}'",
        &s2, &s3
    );
    
    //  We can use the format! macro for string interpolation
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // The format! macro uses references, this call doesn't take ownership of any parameters
    let s = format!("{s1}-{s2}-{s3}");
    println!("s = {s}")
}