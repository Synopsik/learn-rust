# Chapter 7: Managing Growing Projects with Packages, Crates, and Modules

A package can contain ***multiple binary crates*** and optionally ***one library crate***.

Rust has a number of features that allow you to manage your code's organization, including which details are exposed. 
These features, sometimes collectively referred to as the ***module system***, include:

* **Packages**: A Cargo feature that lets you build, test, and share crates
* **Crates**: A tree of modules that produces a library or executable
* **Modules and use**: Let you control the organization, scope, and privacy of paths
* **Paths**: A way of naming an item, such as a struct, function, or module

A ***crate*** is the smallest amount of code that the Rust compiler considers at a time.
A crate can come in one of two forms: a binary crate or a library crate.

---

### ***Binary crates*** 
are programs you can compile to an executable that you can run;
each must have a function called ***main*** that defines what happens when the executable runs.

### ***Library crates*** 
don't have a main function, and they don't compile to an executable.
Instead, they define functionality intended to be shared with multiple projects.

---

The ***crate root*** is a source file that the Rust compiler starts from and makes up the root modules of your crate.

A ***package*** is a bundle of one or more crates that provides a set of functionality.
A package contains a `Cargo.toml` file that describes how to build those crates.
A package must also contain at **least one crate**, whether that's a *library* or *binary crate*.

Let's examine what happens when we create a new project:
```
$ cargo new my-project
    Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```
Here, we have a package that only contains `src/main.rs`, meaning it only contains a binary crate named `my-project`.
If a package contains `src/main.rs` and `src/lib.rs`, it has two crates:
a binary and a library, both with the same name as the package.

A package can have multiple binary crates by placing files in the `src/bin` directory: 
each file will be a separate binary crate.

## Modules Cheat Sheet

### Start from the crate root
When compiling a crate, the compiler first looks in the crate root file 
(usually `src/lib.rs` for a library crate or `src/main.rs` for a binary crate)
for code to compile.

### Declaring modules
In the crate root file, you can declare new modules; say you declare a "garden" module with `mod garden;`.
The compiler will look for the module's code in these places:
* Inline, within curly brackets that replace the semicolon following `mod garden`
* In the file `src/garden.rs`
* In the file `src/garden/mod.rs`

### Declaring submodules
In any file ***other*** than the crate root, you can declare submodules.
For example, you might declare `mod vegetables;` in `src/garden.rs`.
The compiler will look for the submodule's code within the directory named for the parent modules in these places:
* Inline, directly following `mod vegetables`, within curl brackets instead of the semicolon
* In the file `src/garden/vegetables.rs`
* In the file `src/garden/vegetables/mod.rs`

### Paths to code in modules
Once a module is part of your crate, you can refer to code in that module from anywhere else in that same crate,
as long as the privacy rules allow, using the path to the code.
For example, 
an `Asparagus` type in the garden vegetables module would be found at `crate::garden::vegetables::Asparagus`

### Private vs. public
Code within a module is private from its parent modules by default. 
To make a module public, declare it with `pub mod` instead of `mod`.
To make items within a public module public as well, use `pub` before their declarations.

### The `use` keyword
Within a scope, the `use` keyword creates shortcuts to items to reduce repetition of long paths.
In any scope that can refer to `crate::garden::vegetables::Asparagus`,
you can create a shortcut with `use crate::garden::vegetables::Asparagus;` and from then on you only need to write
`Asparagus` to make use of that type in the scope.

## Defining Modules to Control Scope and Privacy

`paths` allow you to name items; `use` brings a path into scope; and the `pub` keyword makes items public.

* ***Modules*** let us organize code within a crate for readability and easy reuse.
* ***Modules*** allow us to control the ***privacy*** of items.
* Code within a module is ***private*** by default.

We can use the restaurant example to illustrate the organization of code.
The *front of house* refers to what customers are able to see and interact with.
The *back of house* is where the chefs and cooks work, where the dishwashers clean, 
and where the managers complete admin duties.

Administrators (our program) can interact with both public and private tools,
while clients can only interact with the public tools.

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
        
        fn seat_at_table() {}
    }
    
    mod serving {
        fn take_order() {}
        
        fn serve_order() {}
        
        fn take_payment() {}
    }
}
```
The module tree as shown:
```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```
This example shows how some modules nest inside other modules;
for example, `hosting` nests inside `front_of_house`.
If *module A* is contained inside *module B*, 
we say that *module A* is the ***child*** of *module B* 
and is rooted under the implicit module named `crate`.

To call a function, we need to know its path. A path can take two forms:

### ***absolute path***
An *absolute path* is the full path starting from a crate root.
```rust
crate::front_of_house::hosting::add_to_waitlist();
```

### ***relative path***
A *relative path* starts from the current module and uses `self`, `super`, or an identifier in the ***current module***.
```rust
front_of_house::hosting::add_to_waitlist();
```

Both *absolute* and *relative* paths are followed by one or more identifiers separated by double colons `::`

To use these functions, they must first be made public or an error will occur.
The error message says that module `hosting` is private.
In Rust, all items (functions, methods, structs, enums, modules, and constants) are private to parent modules by default.
If you want to make an item like a function or struct private, you put it in a module.

This, however, is not enough to access our function. 
We have made the container (module) public, but the contents are still private and hidden.
We must add the `pub` keyword to any public functions, as well as the parent module (container).

```rust
mod front_of_house { // Private module (container) named: front_of_house
    pub mod hosting { // Public module named: hosting
        pub fn add_to_waitlist() {} // Public function named: add_to_waitlist()

        pub fn seat_at_table() {} // Public function named: seat_at_table()
    }

    mod serving { // Private module named: serving
        fn take_order() {} // Private function named: take_order()

        fn serve_order() {} // Private function named: serve_order()

        fn take_payment() {} // Private function named: take_payment()
    }
}

pub fn eat_at_restaurant() { // Public function named: eat_at_restaurant()
    // Absolute path
    // Accessing through: 
    // root -> private front_of_house module ->
    // public hosting module -> public function add_to_waitlist() 
    crate::front_of_house::hosting::add_to_waitlist(); 
    // Relative path
    // Accessing through: current fn -> private front_of_house module 
    // public hosting module -> public function add_to_waitlist()  
    front_of_house::hosting::add_to_waitlist();
}
```
The above library file has allowed us to create a high-level overview of what we are exposing others to use.
This public API is our contract with users of your crate that determines how they can interact with your code.





