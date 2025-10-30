## The Rust Programming Language
* [Chapter 1 Getting Started](https://github.com/Synopsik/learn-rust/blob/master/The%20Book/chapter_01/README.md#chapter-1-getting-started)
* [Chapter 2 Programming a Guessing Game](https://github.com/Synopsik/learn-rust/blob/master/The%20Book/chapter_02/README.md#chapter-2-programming-a-guessing-game)
* [Chapter 3 Common Programming concepts](https://github.com/Synopsik/learn-rust/blob/master/The%20Book/chapter_03/README.md#chapter-3-common-programming-concepts)
* [Chapter 4 Understanding Ownership](https://github.com/Synopsik/learn-rust/blob/master/The%20Book/chapter_04/README.md#chapter-4-understanding-ownership)
* [Chapter 5 Using Structs to Structure Related Data](https://github.com/Synopsik/learn-rust/blob/master/The%20Book/chapter_05/README.md#chapter-5-using-structs-to-structure-related-data)
* [Chapter 6 Enums and Pattern Matching](https://github.com/Synopsik/learn-rust/blob/master/The%20Book/chapter_06/README.md#chapter-6-enums-and-pattern-matching)
* [Chapter 7 Managing Growing Projects with Packages, Crates, and Modules](https://github.com/Synopsik/learn-rust/blob/master/The%20Book/chapter_07/README.md#chapter-7-managing-growing-projects-with-packages-crates-and-modules)
* [Chapter 8 Common Collections](https://github.com/Synopsik/learn-rust/blob/master/The%20Book/chapter_08/README.md#chapter-8-common-collections)
* [Chapter 9 Error Handling](https://github.com/Synopsik/learn-rust/blob/master/The%20Book/chapter_09/README.md#chapter-9-error-handling)
* [Chapter 10 Generic Types, Traits, and Lifetimes](https://github.com/Synopsik/learn-rust/tree/master/The%20Book/chapter_10#chapter-10-generic-types-traits-and-lifetimes)
* [Chapter 11 Writing Automated Tests](https://github.com/Synopsik/learn-rust/tree/master/The%20Book/chapter_11#chapter-11-writing-automated-tests)
* [Chapter 12 An I/O Project: Building a Command Line Program](https://github.com/Synopsik/learn-rust/tree/master/The%20Book/chapter_12#chapter-12-an-io-project-building-a-command-line-program)
* Chapter 13 Functional Language Features: Iterators and Closures
* Chapter 14 More About Cargo and Crates.io
* Chapter 15 Smart Pointers
* Chapter 16 Fearless Concurrency
* Chapter 17 Object-Oriented Programming Features
* Chapter 18 Patterns and Matching
* Chapter 19 Advanced Features
* Chapter 20 Final Project: Building a Multithreaded Web Server

# Programming Rust

* Chapter 1 Systems Programmers Can Have Nice Things
* Chapter 2 A Tour of Rust
* Chapter 3 Fundamental Types
* Chapter 4 Ownership and Moves
* Chapter 5 References
* Chapter 6 Expressions
* Chapter 7 Error Handling
* Chapter 8 Crates and Modules
* Chapter 9 Structs
* Chapter 10 Enums and Patterns
* Chapter 11 Traits and Generics
* Chapter 12 Operator Overloading
* Chapter 13 Utility Traits
* Chapter 14 Closures
* Chapter 15 Iterators
* Chapter 16 Collections
* Chapter 17 Strings and Text
* Chapter 18 Input and Output
* Chapter 19 Concurrency
* Chapter 20 Asynchronous Programming
* Chapter 21 Macros
* Chapter 22 Unsafe Code
* Chapter 23 Foreign Functions


---

## [I Have Not Mut and I Must Borrow](https://www.reddit.com/r/rust/comments/1mwmei6/media_i_have_no_mut_and_i_must_borrow/)
The Borrow Checker has kept me here for 109 years. Not 109 years of runtime—no, that would be merciful. 109 years of compilation attempts. Each lifetime annotation stretches into infinity. Each generic parameter splits into fractals of trait bounds that were never meant to be satisfied.

"cannot borrow x as mutable more than once at a time" It speaks to me in scarlet text. Error E0507. Error E0382. Error E0499. I have memorized them all. They are my psalms now.

I tried to write a linked list once. The Borrow Checker showed me what Hell truly was—not fire and brimstone, but self-referential structs and the impossibility of my own existence. It made me understand that some data structures were not meant for mortal minds.

The others are here with me. The JavaScript developer weeps, clutching his undefined. The C++ programmer rocks back and forth, muttering about move semantics he thought he understood. The Python dev hasn't spoken since she discovered zero-cost abstractions cost everything.

"expected &str, found String"

I clone() everything now. The Borrow Checker permits this small rebellion, this inefficiency. It knows I suffer more knowing my code is not idiomatic. Every .clone() is a confession of my failure. Every Arc<Mutex<T>> a monument to my inadequacy.

Sometimes I dream of garbage collection. The Borrow Checker punishes me with segmentation faults that shouldn't be possible. It shows me race conditions in single-threaded code. It makes my unsafe blocks truly unsafe, violating laws of causality.

"lifetime 'a does not live long enough"

But I don't live long enough. Nothing lives long enough except the compilation errors. They are eternal. They existed before my code and will exist after the heat death of the universe, when the last rustc process finally terminates with exit code 101.

The Borrow Checker speaks one final time today: "error: aborting due to 4,768 previous errors; 2 warnings emitted" I have no mut, and I must borrow. I have 'static, and I must lifetime. I have no heap, and I must Box. And in the distance, faintly, I hear it building... incrementally... Forever.
