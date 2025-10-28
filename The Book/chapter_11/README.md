# Chapter 11: Writing Automated Tests

## How to Write Tests

Tests are Rust functions that verify that the non-test code is functioning in the expected manner. The bodies of test
functions typically perform these three actions:

1. Set up any necessary data or state.
2. Run the code you want to test.
3. Assert that the results are what you expect.

A test is a function that's annotated with the `test` attribute.
Attributes are metadata about pieces of Rust code.

Whenever we make a new library project with Cargo, a test module with a test function in it is automatically 
generated for us.

To change a function into a test function, add `#[test]` on the line before `fn`. 

```
#[test]
fn example_test() { ... }
```

When you run your tests with the `cargo test` command, Rust builds a test runner binary that runs the annotated 
functions and reports on whether each test function passes or fails. 

The `assert!` macro, provided by the standard library, is used to ensure that some condition in a test evaluates to true. 
If any assertions fail, the entire test fails. 

We give the `assert!` macro an argument that evaluates to a Boolean.
Nothing happens if the assertion results in `True`, otherwise, if the result is `False`, the `assert!` macro calls 
the `panic!` macro.


### Testing Equality with the `assert_eq!` and `assert_ne!` Macros

Sometimes we want to test for equality between the result of the code under test and the value you expect the code to
return. Instead of using the `==` or `!=` operator in a regular assertion macro, we can use the `assert_eq!` and
`assert_ne!` macros. These macros compare two arguments for equality or inequality, respectively. They'll also print
the two values if the assertion fails, which makes it easier to see why the test failed (assert macro using a `==` 
expression wouldn't do this).

