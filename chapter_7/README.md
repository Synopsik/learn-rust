# Chapter 7: Managing Growing Projects with Packages, Crates, and Modules

A package can contain ***multiple binary crates*** and optionally ***one library crate***.

Rust has a number of features that allow you to manage your code's organization, including which details are exposed. 
These features, sometimes collectively referred to as the module system, include:

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