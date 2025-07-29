# The Rust Programming Language
This book and repo assumes Rust version 1.62.0 or later

## Progress
* ~~Chapter 1 Getting Started~~
* ~~Chapter 2 Programming a Guessing Game~~
* ~~Chapter 3 Common Programming concepts~~
* ~~Chapter 4 Understanding Ownership~~
* Chapter 5 Using Structs to Structure Related Data
* Chapter 6 Enums and Pattern Matching
* Chapter 7 Managing Growing Projects with Packages, Crates, and Modules
* Chapter 8 Common Collections
* Chapter 9 Error Handling
* Chapter 10 Generic Types, Traits, and Lifetimes
* Chapter 11 Writing Automated Tests
* Chapter 12 An I/O Project: Building a Command Line Program
* Chapter 13 Functional Language Features: Iterators and Closures
* Chapter 14 More About Cargo and Crates.io
* Chapter 15 Smart Pointers
* Chapter 16 Fearless Concurrency
* Chapter 17 Object-Oriented Programming Features
* Chapter 18 Patterns and Matching
* Chapter 19 Advanced Features
* Chapter 20 Final Project: Building a Multithreaded Web Server

# Notes

## Chapter 1
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

## Chapter 2
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

## Chapter 3
The compiler is able to evaluate a limited number of operations (expression) types at runtime. 

For example: `const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;`

Even though we are declaring a constant variable, we can do it this way because it's made easier to write 
3 hours times 60 minutes times 60 seconds in an expression compared to hardcoded as 10800 for the seconds.

---

We can use Shadowing to "mutate" an immutable variable by calling a new variable to replace it. 
In doing this we can change the variable type:
```
let variable = " ";
let variable = variable.len();
```

However, if we try to mutate a variable type with a mutable var, we get an error:
```
let mut variable = " ";
variable = variables.len();
// ERROR!!
```
The error says we cannot mutate the variable type.

---

#### Integer Types in Rust
| Length  | Signed  | Unsigned | Size         |
|---------|---------|----------|--------------|
| 8-bit   | `i8`    | `u8`     | 1 byte       |
| 16-bit  | `i16`   | `u16`    | 2 bytes      |
| 32-bit  | `i32`   | `u32`    | 4 bytes      |
| 64-bit  | `i64`   | `u64`    | 8 bytes      |
| 128-bit | `i128`  | `u128`   | 16 bytes     |
| arch    | `isize` | `usize`  | 4 or 8 bytes |
| 32-bit  | `f32`   | n/a      | 4 bytes      |
| 64-bit  | `f64`   | n/a      | 8 bytes      |

### Formula to find a data types **min to max** size, **-(2<sup>n-1</sup>) to 2<sup>n-1</sup>-1** inclusive.

When we are estimating an **i8** for **n** we result with -(2<sup>**7**</sup>) to 2<sup>**7**</sup>-1 which equals **-128 to 127**.

*Unsigned variants* can store from **0 to 2<sup>**n**</sup>-1** inclusive.

For a u8 we can store from 0 to 2<sup>**8**</sup>-1, which equals **0 to 255**.

`isize` and `usize` depends on the architecture of the computer that the program is running on

#### Other Types in Rust
| Type | Size    |
|------|---------|
| Char | 4 bytes |
| Bool | 1 byte  |

Arrays must have a fixed length when created.
```
let arr: [i32; 5] = [1, 2, 3, 4, 5];  // 20 bytes (5 × 4 bytes)
let bytes: [u8; 10] = [0; 10];        // 10 bytes
```

#### Integer Literals in Rust
| Number literals | Example       | Default Type | Size    |
|-----------------|---------------|--------------|---------|
| Decimal         | `98_222`      | i32          | 4 bytes |
| Hex             | `0xff`        | i32          | 4 bytes |
| Octal           | `0o77`        | i32          | 4 bytes |
| Binary          | `0b1111_0000` | i32          | 4 bytes |
| Byte (u8 only)  | `b'A'`        | u8           | 1 byte  |

#### Heap Allocated
```
let s: String = String::from("hello");  // 24 bytes on stack (ptr + len + capacity)
                                        // + heap allocation for actual string data
let str_ref: &str = "hello";            // 16 bytes (pointer + length)
```

### Pointer Sizes

#### Regular Reference
```
let x = 42;
let r: &i32 = &x;      // 8 bytes on 64-bit systems (just a pointer)
let mr: &mut i32;      // 8 bytes on 64-bit systems
```

#### Fat Reference (DST Reference)
```
let s: &str = "hello";           // 16 bytes (8-byte pointer + 8-byte length)
let slice: &[i32] = &[1, 2, 3];  // 16 bytes (8-byte pointer + 8-byte length)
let obj: &dyn Display = &42;     // 16 bytes (8-byte pointer + 8-byte vtable pointer)
```


---

### Evaluating Expressions

Expressions alone do not need to include ending semicolons (`;`), 
```
let y = {
    let x = 3; // Statement, semicolon
    x + 1 // Expression, doesn't require a semicolon
}; // Main Statement, semicolon
```
it's only when paired with a statement that you need to close with a semicolon:
`let x = x + 1;`

---

### Loop Labels

Loop labels can be used to break a specific loop at any point in the nested hierarchy.
```
'outer_loop: loop {
    `inner_loop: loop {
        break `outer_loop;
    }
}
```

While loops can be slower than for loops because of the additional check for validity during each iteration

Using a For loop instead, we can increase the safety of the code by removing the chance for the index to either
go beyond the end of the array or not go far enough and miss some items.
```
let a = [10, 20, 30, 40, 50]

for element in a {
    println!("{element}");
}
```

Therefore, ONLY use while loop you are absolutely uncertain how many iterations are needed

## Chapter 4

