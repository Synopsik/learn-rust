# Chapter 5: Using Structs to Structure Related Data

## Defining A Struct

In a struct you name each piece of data, so it's clear what the value means.
This makes structs more flexible than tuples because you don't have to rely on an order to access each instance, 
similar to a dictionary.

---

To define a struct you enter the keyword `struct` and name the entire struct.

```
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

---

## Using A Struct

To use the struct (after defining it, duh!) create an instance by specifying valid concrete values for each field.
You must state the name followed by curly brackets that contain `key:value` pairs, 
where the keys are the names of the fields and the values are the data we want to store in those fields.

```
let user1 = User {
    active: true,
    username: String::from("someusername123"),
    email: String::from("someone@example.com"),
    sign_in_count: 1,
};
```

---

We use dot notation to access specific values from a struct.
For example, we use `user1.email` to return the email string.

```
println!(user1.email);
```

---

## Updating A Struct

If the instance is mutable, we can change a value by using the dot notation and assigning into a particular field.

```
user1.email = String::from("anotheremail@example.com");
```

Note, the entire instance must be mutable; Rust doesn't allow us to mark only certain fields as mutable.

---

If the parameter names and the struct field names are exactly the same in the previous example,
we can use ***field init shorthand*** syntax to rewrite `build_user` so it behaves the same without the repetition.

```
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```

The `username` and `email` parameters have the same name as struct fields.

---

## Struct Update Syntax

Using the struct update syntax, 
you can create an instance using another instance while specifying only the relevant values.

Instead of doing:
```
let user2 = User {
    active: user1.active,
    username: user1.username,
    email: String::from("another@example.com"),
    sign_in_count: user1.sign_in_count,
};
```

We should do:
```
let user2 = User {
    email: String::from("another@example.com"),
    ..user1
};
```

Using ***struct update syntax*** we get the same results with less code.
The syntax `..` specifies that the remaining fields not explicitly set should have the same values as the fields in the 
given instance. 

Also, we can now no longer use `user1` after creating `user2` because the String in the `username` field
was moved into `user2`. If instead we had created a new string for `username` so that both String values were replaced, 
only `active` and  `sign_in_count` would be used from `user1`. Since bools and int data types inherit the Copy trait, 
`user1` would still be valid.

---

## Unit-Like Structs

Structs can also be defined that do NOT have any fields. 
These can be useful when creating a trait that doesn't hold data.

```
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

---

## Debug and Displaying Structs

The `dbg!` macro is similar to the `println!` macro except that it takes ownership of an expression. 
It then prints the file and line number where the `dbg!` is along with the resultant value of that expression, 
finally, ownership of the value is returned.

```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1);
}
```
Output:
```
[src/main.rs:10] 30 * scale = 60
[src/main.rs:14] &rect1 = Rectangle {
    width: 60,
    height: 50,
}
```

---

Structs don't implement Display by default; there are many different ways to print the display possibilities.
That said, we can enable a Debug trait to display the results using `{:?}` inline 
and adding the outer attribute `#[derive(Debug)]` to our struct.

```
#[derive(Debug)]
struct Rectangle {
   width: u32,
   height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!("rect1 is {:?}", rect1);
}

```
Output:
```
rect1 is Rectangle { width: 30, height: 50 }
```

---

We can make our output a bit easier to read using `{:#?}` instead

Output:
```
rect1 is Rectangle {
    width: 30,
    height: 50,
}
```

---

## Method Syntax

To define the method in the context of Rectangle, we start an `impl` (implementation) block.
Everything within this `impl` block is associated with the Rectangle type.
Inside the block we can create functions, but we must always use self as the first parameter.

```
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

We use ***method syntax*** to call our area method

```
rect1.area()
```

We can also name our method after one of our struct's fields.
This way when we follow `rect1.width` with parentheses `()` Rust knows we mean the method `width`.
When we don't use parentheses, Rust knows we mean the field `width`.
    
