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

In the example below, we want to use `s2` after appending its contents to `s1`:
```
let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);
println!("s2 is {s2}");
```
If the `push_str` method were to take ownership of `s2`, we wouldn't be able to print its value on the last line.

---

The `push` method takes a single char as a parameter and adds it to the `String`:
```
let mut s = String::from("lo");
s.push('l');
```
---

We can also use the `+` operator to concatenate two existing strings.
```
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
```
The `+` operator uses the `add` method, whose signature looks something like this:
```
fn add(self, s: &str) -> String { ... }
```
A couple things to note, first:  
* `s2` has an `&`, meaning that we're adding a ***reference*** of the second string to the first string.  
* This is because of the `s` param in the `add` function: 
  * we can only add a `&str` to a `String` 
  * we can't add two `String` values together.                      
* We can use `&s2` because the compiler can *coerce* the `&String` argument into a `&str`
  * *deref coercion* turns `&s2` into `&s2[...]`
  * Ownership of `&s2` is NOT transferred
  * `s2` will be a valid `String` after this operation

Second, as seen in the function signature:
* `add` takes ownership of `self` because there is no `&`
  * `s1` transfers ownership to `add`
  * `s1` will no longer be valid after transferring ownership

So this statement actually takes ownership of `s1`, appends a copy of the contents of `s2`, 
and then returns ownership of the result. This looks like it's making a lot of copies, but it isn't;
the implementation is more efficient than copying.

---

The `format!` macro uses references so that this call doesn't take ownership of any of its parameters:
```
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{s1}-{s2}-{s3}");
```

---

### Indexing into Strings
If you try to access parts of a `String` using indexing syntax in Rust, you'll get an error.
Let's look at this invalid code and explain why:
```
let s1 = String::from("hello");
let h = s1[0];

error[E0277]: the type `String` cannot be indexed by `{integer}`
```
---

A `String` is a wrapper over a `Vec<u8>`. Let's examine a properly encoded UTF-8 example string:
```
let hello = String::from("Hola");
```
In this case, `len` will be 4, which means the vector storing the string "Hola" is four bytes long.
Each of these letters takes **one byte** when encoded in UTF-8.
The issue with this is that some languages and symbols require more than one byte for each letter.



The capital Cyrillic letter *Ze* `З` seems similar to the number `3`, 
however, each letter in this language requires ***two bytes***. 

Therefore, an index into the string's bytes will not always correlate to a valid Unicode scalar value.
Consider the example Rust code:
```
let hello = "Здравствуйте";
let answer = &hello[0];
```
If this code was valid, we know that the first byte would not be `З`, the first letter.

When encoded in UTF-8 the first byte of `З` is 208 and the second is 151. 
So `answer` would be 208, but 208 is not valid on its own.

Users generally don't want the byte value returned, even if the string contains only Latin letters:
if `&"hello"[0]` were valid code that returned the byte value, it would return 104 not h.

Rusts answer is to not compile this code at all and prevent these misunderstandings early in the development process.
(maybe unsafe mode?)

---

// TODO: Implement "Bytes and Scalar Values and Grapheme Clusters! Oh My!"

---

A final reason Rust doesn't allow us to index into a `String` to get a character is that indexing operations are 
expected to always take constant time (O(1)). But it isn't possible to guarantee that performance with a `String`,
because Rust would have to walk through the contents from the beginning to the index to determine how many valid 
characters there were.

### String Slicing
Indexing into a `String` is often a bad idea because it's not clear what the return type of the string-indexing 
operation should be: a byte value, a character, a grapheme cluster, or a string slice. If you really need to do this,
Rust offers you string slicing to be specific with what you want.

You use a range to create a string slice containing particular bytes:
```
let hello = "Здравствуйте";

let s = &hello[0..4];
```
`s` will be a `&str` that contains the first four bytes of the string.
If we try to slice only part of a character's bytes with something like `&hello[0..1]`,
Rust would panic at runtime in the same way as if an invalid index were accessed in a vector:

```
thread 'main' panicked at src/main.rs:4:19:
byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`
```
Always **USE CAUTION** when creating string slices with ranges.
Any mistakes will crash your program.

---

### Methods for Iterating Over Strings
The best way to operate on pieces of strings is to be explicit about whether you want characters or bytes.
* Unicode scalar values
  * `chars` method - calling `chars` on "Зд" separates out and returns two values of type `char`,
  and you can iterate over the result to access each element:
  ```
  for c in "Зд".chars() {
    println!("{c}");
  }
  ```
  * This prints:
  ```
  З
  д
  ```
* Raw bytes
  * `bytes` method - calling `bytes` will return each raw byte:
  ```
  for b in "Зд".bytes() {
    println!("{b}");
  }
  ```
  * This prints the four bytes that make up the string:
  ```
  208
  151
  208
  180
  ```
**It's important to remember that valid Unicode scalar values may be made up of more than one byte.**
