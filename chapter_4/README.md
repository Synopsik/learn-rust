# Chapter 4: Understanding Ownership

## Notes

### Rust's three main rules of ownership

1. Each **value** in Rust has an ***owner***
2. There can only be **one** ***owner*** at a time
3. When the ***owner*** goes out of scope, the **value** will be dropped

---

String literals are allocated on the stack.
They are fast and efficient because we know their size at compile time.

A string literal is hardcoded in the final executable and is ***immutable***.

```
let s = "Hello World!";
```

While,

a `String` allocates memory on the heap instead of on the stack and is ***mutable***

The *double colon* operator `::` allows us to namespace this particular `from` function under the `String` type

```
let s = String::from("Hello World!");
```

---

## Memory and Allocation


With a `String` type, the value is ***mutable*** and growable.
Therefore, we need to allocate an amount of memory on to the heap.

Using memory allocator requires two steps:
1. Memory must be ***requested*** from the **memory allocator** at runtime
2. When we are done with this memory, we need to ***return*** it to the allocator

In Rust, we don't need to worry about garbage collection;
when a variable goes out of scope, the memory is automatically returned.

---

## Variables

Multiple ***variables*** are able to interact with the same **data** in many different ways.

For example, a variable that is referencing another variable in heap memory.

```
s1 = String::from("hello");
s2 = s1;
```

---

The `String` is made up of three parts: a pointer, a length, and a capacity.

### s1

| Name       | Value                                     |     
|------------|-------------------------------------------|
| `ptr`      | Pointer to heap memory address at index 0 |
| `len`      | 5                                         |
| `capacity` | 5                                         |

The table above is stored on the stack after `s1 = String::from("hello");`

### Heap memory

| Index | Value |
|-------|-------|
| 0     | h     |
| 1     | e     |
| 2     | l     |
| 3     | l     |
| 4     | o     |

The table above in stored on the heap.

---

## Move

As stated in the main rules of ownership, each variable can only have one owner at a time.
When we assign s1 to s2, Rust copies the value to s2 and invalidates s1.
This seems like a shallow copy, but it called a ***move*** on the second line
`s2 = s1;`

### s1

| Name       | Value   |     
|------------|---------|
| `ptr`      | Invalid |
| `len`      | Invalid |
| `capacity` | Invalid |

### s2

| Name       | Value                                     |     
|------------|-------------------------------------------|
| `ptr`      | Pointer to heap memory address at index 0 |
| `len`      | 5                                         |
| `capacity` | 5                                         |

### Heap memory

| Index | Value |
|-------|-------|
| 0     | h     |
| 1     | e     |
| 2     | l     |
| 3     | l     |
| 4     | o     |

--- 

## Clone

If instead you intend to make a deep copy, `clone` can be used to create a duplicate.
**This operation may be expensive.**

### s1

| Name       | Value                                     |     
|------------|-------------------------------------------|
| `ptr`      | Pointer to heap memory address at index 0 |
| `len`      | 5                                         |
| `capacity` | 5                                         |

### Heap memory

| Index | Value |
|-------|-------|
| 0     | h     |
| 1     | e     |
| 2     | l     |
| 3     | l     |
| 4     | o     |

---

### s2

| Name       | Value                                            |     
|------------|--------------------------------------------------|
| `ptr`      | Pointer to cloned heap memory address at index 0 |
| `len`      | 5                                                |
| `capacity` | 5                                                |

### Cloned Heap memory

| Index | Value |
|-------|-------|
| 0     | h     |
| 1     | e     |
| 2     | l     |
| 3     | l     |
| 4     | o     |

Both ***s1*** and ***s2*** have separate copies of the string `"hello"` in heap memory.

---

## Data Types that DON'T follow these rules

When it comes to data types that have a definition size before compilation occurs,
these are always copied instead of moved since they are stored on the stack and quick to make.
These data types have a trait called Copy.
The data types the implement this are:

* All integer types
* Floating-Point types
* Boolean types 
* Character type
* Tuples with the exception that they only contain data types with the Copy trait (i32, i32) 

We can also use a tuple to return multiple values.

---

## References and Borrowing

If you don't want to transfer ownership and still use a value, we need to use a reference.

A ***reference*** is like a pointer in that it's an address we can follow to access the data stored at that address.
This data is owned by another variable. 
A ***reference*** is also guaranteed to point to a valid **value** of a particular **type** for the life of that reference.

Here is a function that has a reference to an object as a parameter instead of taking ownership of the value:
```
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

---

A ***reference*** allows you to refer to some value without taking ownership of it.

### s

| name | value                              |
|------|------------------------------------|
| ptr  | Pointer to s1 variable ptr address |

### s1

| name     | value                                     |
|----------|-------------------------------------------|
| ptr      | Pointer to heap memory address at index 0 |
| len      | 5                                         |
| capacity | 5                                         |

### Heap memory

| index | value |
|-------|-------|
| 0     | h     |
| 1     | e     |
| 2     | l     |
| 3     | l     |
| 4     | o     |

---

## Mutable References

With a regular immutable reference, we can't change the contents of what we're referencing.

To modify a borrowed value, we need to create a ***mutable reference*** by using the **&** operator.

```
fn main() {
    let mut s = String::from("Hello");
    
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}
```

Mutable reference have one big restriction: 
once you have a mutable reference to a value, you can have no other references to that value.

For example this code will fail:

```
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{r1}, {r2}");
```

The restriction preventing multiple mutable references to the same data at the same time
allows for mutations but in a very controlled manner.

---

## Rules of References

* At any given time, you can have either **one** ***mutable reference*** or **any number** of ***immutable references***
* Reference must always be valid

---

## Slice Type

***Slices*** let you reference a contiguous sequence of elements in a collection rather than the whole collection.
A slice is a kind of reference, so it does not have ownership.

```
let s = String::from("Hello world!);

let hello = &s[0..5];
let world = &s[6..12];
```