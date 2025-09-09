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

***Binary crates*** are programs you can compile to an executable that you can run;
each must have a function called ***main*** that defines what happens when the executable runs.

***Library crates*** don't have a main function, and they don't compile to an executable.
Instead, they define functionality intended to be shared with multiple projects.

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

~~~~
