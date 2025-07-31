# Chapter 1: Getting Started

## Notes
Rust is amazing because it also installs all of its documentation for offline reading.
Your language comes pre-built with a how-to book; how cool is that! Using `rustup doc`
you can read *The Rust Programming Language* book, look up any types, functions, or anything else provided by the standard library.

Rust also comes with an automatic formatter tool called `rustfmt`, this can help to keep
your code style consistent across projects.

Rust uses macros, to print to console you would use `println!();`, the `!` denotes a macro being used.

Packages of code (libraries) are called `crates`.

`cargo check` is faster than `cargo build` because it skips producing an executable
and only confirms successful compilation.

Releases need to be built longer for optimizations, to do this, use `cargo build --release`.