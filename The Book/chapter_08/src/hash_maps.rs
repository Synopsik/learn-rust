use std::collections::HashMap;
pub fn run_examples() {
    hash_map_creation();
    hash_map_ownership();
    hash_map_overwrite_value();
    hash_map_add_maybe_new_key();
    hash_map_updating_value();
}

pub fn hash_map_creation() {
    println!("\nCreating a New Hash Map");
    // Inits empty scores hash map
    let mut scores = HashMap::new();
    println!("Init: {:?}", scores);
    // Inserts scores into teams
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("Created teams Blue & Yellow: {:?}", scores);


    println!("\nAccessing Values in a Hash Map");
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}

fn hash_map_ownership() {
    println!("\nHash Maps and Ownership");
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    println!("field_name: {field_name}\nfield_value: {field_value}");
    
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("{:?}", map);
    // field_name and field_value are now invalid
}

fn hash_map_overwrite_value() {
    let mut scores = HashMap::new();
    
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // Duplicate hash key
    
    println!("{:?}", scores);
}

fn hash_map_add_maybe_new_key() {
    let blue_team = String::from("Blue");
    let yellow_team = String::from("Yellow");
    let mut scores = HashMap::new();
    scores.insert(&blue_team, 10);
    
    scores.entry(&yellow_team).or_insert(50);
    scores.entry(&blue_team).or_insert(50); // Existing entries fail silently
    
    println!("{:?}", scores);
}

fn hash_map_updating_value() {
    let text = "hello world wonderful world wonderful world";
    
    let mut map = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("{:?}", map);
}