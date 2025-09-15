# Chapter 8: Common Collections

## Vectors
Vectors allows us to store more than one value in a single data structure 
that puts all the values next to each other in memory.
Vectors can only store values of the same type.

### Creating a New Vector
Rust provides the `vec!` macro, which will create a new vector that holds the values you give it.
```
let v = vec![1, 2, 3];
```
Because, we've given initial `i32` values, Rust can infer that the type of `v` is `Vec<i32>`,
and the ***type annotation*** isn't necessary.

### Updating a Vector
To add elements to a ***vector***, we use the `push` method.
With any variable, we must use the `mut` keyword if we want to *change the value*.
```
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7); 
v.push(8);
```
Since all the numbers we push are of type `i32`, Rust infers this and we don't need the `Vec<i32>` annotation.

### Reading Elements of Vectors
There are two ways to reference a value stored in a vector: **indexing** or using the **get method**.
```
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {third}");

let third: Option<&i32> = v.get(2);
match third {
    Some(third) => println!("Thie third element is {third}"),
    None => println!("There is no third element"),
}
```
Rust provides these two ways to reference an element, so you can choose 
how the program behaves when you try to use an index value outside the range of existing elements.

#### Indexing
Indexing is best used when you want your program to crash 
if there's an attempt to access an element past the end of the vector.
The first `[]` method that fails will cause the program to panic because it references a nonexistent element.

#### Get Method
Get Method is best if accessing an element beyond the range of the vector may happen occasionally.
For example, if a person accidentally enters a number that's too large,
the program gets a `None` value. You could tell the user how many items are in the current vector
and give them another chance to enter a valid value.

### Iterating Over the Values in a Vector
Use a `for` loop to access each element in a vector.

Here's how to use a `for` loop to get immutable references to each element in a vector and print them:
```
let v = vec![100, 32, 57];
for i in &v {
    println!("{i}");
}
```
Here's how to use a `for` loop to get mutable references to each element and adds a number to them:
```
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}
```
We use the `*` ***dereference operator*** to get to the value in `i` before we can use the `+=` operator.

### Using an Enum to Store Multiple Types
We use enums when we need **one type** to represent elements of ***different types***.
We can define an enum whose variants will hold the different value types,
and all the enum variants will be considered the same types: that of the enum.

Then we can create a vector to hold that enum and so, ultimately, hold different types.
```
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
}
```

## Strings
Strings are implemented as a collection of bytes, 
plus some methods to provide useful functionality when those bytes are interpreted as text.

* #### **Core Language:**    
Rust core only has one string type, the string slice `str`

* #### **Standard Library:**    
The Rust standard library has another string type. 
The `String` is a growable, mutable, owned, UTF-8 encoded string type.

Both `String` and `str` slices are UTF-8 encoded

### Creating a New String
`String` is actually implemented as a wrapper around a vector of bytes with some extra 
guarantees, restrictions, and capabilities.

An example of a function that works the same way with `Vec<T>` and `String` is the `new` function to create an instance:
```
let mut s = String::new();
```
Often we'll have some initial data with which we want to start the string.
For that, we use the `to_string` method. 
This is on any type that implements the `Display` trait, like the string literals in this example:
```
let data = "initial contents";

let s = data.to_string();

// The method also works on a literal directly:
let s = "initial contents".to_string();
```
We can also use the function `String::from` to create a `String` from a string literal.
```
let s = String::from("initial contents");
```
In these examples, `String::from` and `to_string` do the same thing.

### Updating a String
We can grow a `String` by using the `push_str` method to append a ***string slice***.
```
let mut s = String::from("foo");
s.push_str("bar");
```
The `push_str` method takes a string slice because we don't necessarily want to take ownership of the parameter.
