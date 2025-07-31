# Chapter 2: Programming a Guessing Game

## Notes

Rust has a standard library imported into every program (std::io)
these can be found at https://doc.rust-lang.org/std/prelude/index.html.

When using an expression with a print statement, use empty curly brackets `{}` in the string literal
and follow the string with a comma-separated list of your expression. For example:
```
let x = 5;
let y = 10;

println!("x = {x} and y + 7 = {}", y + 7);
```

A great feature from Cargo is the `cargo doc --open` command that builds documentation
from your current project and opens it in your browser. Each crate has documentation with instructions,
so you can study the methods and functions for each package locally installed.

When taking user input from the console, remember that the input is the keyboard strokes
and the newline character created by pressing Enter. So if a user types `5` and presses Enter,
the result will look like `5\n`. This is why we need to use the `.trim()` method to remove any leading
or following whitespaces, `\n`, or `\r\n`.