use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};
pub fn run_all() {
    // try_open_file();
    // matching_multiple_errors();
    // matching_shortcuts();
    propagating_errors();
    new_operator();
}

fn try_open_file() {
    let greeting_file_result = File::open("../hello.txt");
    
    let greeting_file = match greeting_file_result {
        Ok(file) => file, // When File::open succeeds, Ok() is a valid instance
        Err(error) => { // When File::open fails, Err() is a valid instance
            panic!("Problem opening the file: {:?}", error);
        }
    };
}

fn matching_multiple_errors() {
    let greeting_file_result = File::open("../hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // We can match on any ErrorKind:: type for controlled error handling
            ErrorKind::NotFound => { 
                match File::create("../hello.txt") {
                    // Since this may fail, we need some err handling
                    Ok(fc) => fc,
                    Err(e) => panic!(
                        "Problem creating the file: {:?}", e
                    ),
                }
            },
            // ErrorKind::******* replace stars with type
            other_error => { // Default error state
                panic!(
                    "Problem opening the file: {:?}", other_error
                );
            },
        },
    };
}

fn matching_shortcuts() {
    let greeting_file = File::open("../hello.txt").unwrap();
    
    let greeting_file_2 = File::open("../hello.txt")
        .expect("hello.txt should be included in this project");
}

fn propagating_errors() {
    let username = read_username_from_file();
    
    println!("{:?}", username);
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("../hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn new_operator() {
    let username1 = updated_read_username_from_file();
    let username2 = shorter_read_username_from_file();
    let username3 = standard_read_username_from_file();
    let last_character = last_char_of_first_line("Hello World");
    
    println!("{:?}::{:?}::{:?}::{:?}",
             username1.unwrap(),
             username2.unwrap(),
             username3.unwrap(),
             last_character.unwrap(),
    );
}

fn updated_read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("../hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn shorter_read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("../hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn standard_read_username_from_file() -> Result<String, io::Error> {
    // Common operation so std provides convenience functions to do all the work
    fs::read_to_string("../hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    /*
    This function returns `Option<char>` because it's possible there may or may not be a char.
    We take the `text` string slice and call the lines method to create an iterator over the lines 
    in the string. because this function wants to examine the first line, it calls `next` on the
    iterator to get the first value from the iterator. If `text` is the empty string, this call to 
    `next` will return `None`, in which case we use ? to stop and return `None` from our func.
    If `text` is not the empty string `next` will return a `Some` value containing a string slice
    of the first line in `text`.
     */
    text.lines().next()?.chars().last()
}