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
return. 

Instead of using the `==` or `!=` operator in a regular assertion macro, we can use the `assert_eq!` and `assert_ne!` 
macros. These macros compare two arguments for equality or inequality, respectively. They'll also print the two values 
if the assertion fails, which makes it easier to see why the test failed 
(assert macro using a `==` expression wouldn't do this). 

When the assertation fails, these macros print their arguments using debug formatting, which means the values being 
compared must implement the `PartialEq` and `Debug` traits. Since both traits are derivable traits, this is as simple 
as adding the `#[derive(PartialEq, Debug)]` annotation to your struct or enum definition.

### Adding Custom Failure Messages

You can also add a custom message to be printed with the failure message as optional arguments to the `assert!`,
`assert_eq!`, and `assert_ne!` macros. Any arguments specified after the required arguments are passed along to the 
`format!` macro, so you can pass a format string that contains `{}` placeholders and values to go in those placeholders.

```
#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{result}`"
    };
}
```

We will then be able to see the value we actually got in the test output, which would help debug what happened
instead of what we expected to happen.

### Checking for Panics with `should_panic`

It's also important to check that our code handles error conditions as we expect. For example, a custom `Guess` type
we've created has the guarantee that an instance will only contain values between 1 and 100. We can write a test that
ensures that attempting to create a `Guess` instance with a value outside that range panics.

We do this by adding the attribute `#[should_panic]` to the test function. The test will pass if the function panics,
otherwise the test will fail if the function doesn't panic.

```
#[test]
#[should_panic]
fn panic_test_example() {
    panic!();
}
```

We place the `#[should_panic]` attribute after the `#[test]` attribute and before the test function it applies to.
The main downside of this attribute is that it can be imprecise, a `should_panic` test would pass even if the test 
panics for a different reason from the one we were expecting.

We can add an optional `expected` parameter to the `should_panic` attribute to make it more precise. The test harness
will make sure that the failure message contains the provided text.

```
#[test]
#[should_panic(expected = "substring of expected panic message")]
fn panic_test_example() -> i32 {
    15
}
```

This test will only pass if the substring we put in the `expected` parameter, is a valid substring of the message in 
the `panic!()` that the calling function uses.

```
pub fn foo(value: i32) {
    if value < 0 {
        panic!("Value must be greater than 0, got {}", value);
    } else if value > 10 {
        panic!(Value must be less than 10, got {}", value);
    }
}

#[test]
#[should_panic(expected="less than 10")]
fn greater_than_10() {
    // foo(-15); This would fail since it's not what we're expecting
    foo(15);
}
```

### Using `Result<T, E>` in Tests

Tests can also use `Result<T, E>` to return an `Err` instead of panicking.

```
#[test]
fn it_works() -> Result<(), String> {
    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err(String::from("Two plus two does not equal four"))
    }
}
```

Rather than calling the assertion macros, we return `Ok(())` when the test passes and an `Err` with a `String`
inside when the test fails. This also allows you to use the `?` operator in the body of tests, which can be a 
convenient way to write tests that should fail if any operation within them returns an `Err` variant.

You CAN'T use the `#[should_panic]` annotation on tests that use `Result<T, E>`.

To assert that an operation returns an `Err` variant, _don't_ use the `?` operator.
Instead, use `assert!(value.is_err())`.

## Controlling How Tests Are Run

`cargo test` compiles your code in test mode and runs the resultant test binary.
The default behavior of the binary produced by `cargo test` is to run all the tests in parallel and capture output 
generated during test runs, preventing the output from being displayed and making it easier to read the output related
to the test results.

Some command line options can go to `cargo test`, while some go to the resultant test binary.

To separate these two types of arguments, you list the arguments that go to `cargo test` followed by the separator
`--` and then the ones that go to the test binary.

* `cargo test --help`: Displays the options you can use with `cargo test`
* `cargo test -- --help`: Displays the options you can use after the separator with the test binary.

### Running Tests in Parallel or Consecutively

Because the tests are running at the same time, you must make sure your tests don't depend on each other or on any
shared state, including a shared environment, such as the current working directory or environment variables.

For example, if each of your tests runs some code that creates a file on disk named `test-output.txt` and writes some
data to that file, Then each test reads the data in that file and asserts that the file contains a particular value, 
which is different in each test. Because the tests run at the same time, one test might overwrite the file in the time 
between another test writing and reading the file.

The second test will fail, not because the code is incorrect but because the tests have interfered with each other
while running in parallel. The solution is to either write to separate files or run the tests synchronously.

If you don't want to run the tests in parallel, or if you want to modify the count of threads, use the `--test-threads`
flag and the number of threads you want to use for the test binary.

```
cargo test -- --test-threads=1
```

If we set the number of test threads to 1, this tells the program not to use any parallelism.

### Showing Function Output

By default, if a test passes, the test library captures anything printed to standard output.
So if we call `println!` in a test and: 
* The test passes, we **WON'T** see the `println!` output in the terminal.
* The test fails, we **WILL** see the `println!` output in the terminal next to the failure message.

If we want to see printed values for passing tests as well, we can tell Rust to also show the output of successful
tests with `--show-output`

```
cargo test -- --show-output
```

### Running a Subset of Tests by Name

Running a full test suite can take a long time. You might want to run only the tests pertaining to that code. You can 
choose which tests to run by passing `cargo test` the name or names of the test(s) you want to run as an argument.

If we want to run a single test, pass the name of any test function to `cargo test`

```
cargo test one_hundred
```

All the tests with the substring `one_hundred` in their name ran.
All other tests are displayed as `2 filtered out` with the number being the tests that didn't run.

---

If we want to run multiple tests, we use part of a test name. Then, any test whose name matches that value will be run.

> [!Note]
> The module in which a test appears becomes part of the test's name, so we can run all the tests in a module by
> filtering on the module's name.

---

Some tests can be very time-consuming to execute, so you might want to exclude them during most runs of `cargo test`.
Instead of creating arguments for all the test's you don't want run, you can annotate the time-consuming tests using
the `ignore` attribute to exclude them:

```
#[test]
#[ignore]
fn expensive_test() {
    // Code that takes a long time to run
}
```

After `#[test]`, we add the `#[ignore]` line to the test we want to exclude.

If we want to run only the ignored tests, we can use: 

```
cargo test -- --ignored
```

If we want to run all tests whether they're ignored or not, you can run

```
cargo test -- --include-ignored
```

## Test Organization

The Rust community thinks about tests in terms of two main categories: **unit tests** and **integration tests**.
* _Unit tests_: Small and more focused, testing one module in isolation at a time, and can test private interfaces.
* _Integration tests_: Entirely external to your library and use your code in the same way any other external code
would, using only the public interface and potentially exercising multiple modules per test.

### Unit Tests

You'll put unit tests in the `src` directory in each file with the code that they're testing.
The convention is to create a module named `tests` in each file to contain the test function and to annotate the 
module with `cfg(test)`.

The `#[cfg(test)]` annotation on the `tests` module tells Rust to compile and run the test code only when you run 
`cargo test`, not when you run `cargo build`. The tests are also not included in the resultant compiled artifact when
placed in this module. This annotation is needed because unit tests go in the same files as the code, integration tests,
on the other hand, go in a different directory, so they don't need the `#[cfg(test)]` annotation.

> [!Note]
> Private functions can also be freely tested, as long as you (`use super::*;` to) bring the function into scope.
> Functions do NOT need to be marked `pub` in order to test them.
 
### Integration Tests

Integration tests are entirely external to your library. They can only call functions that are part of your library's
public API. Their purpose is to test whether many parts of your library work together correctly. Units of code that
work correctly on their own could have problems when integrated, so test coverage of the integrated code is important
as well.

To create integration tests, you first need to create a `tests` directory at the top level of our project directory, 
next to `src`. Cargo knows to look for integration test files in this directory. We can then make as many test files as
we want, and Cargo will compile each of the files as an individual crate.

```
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_tests.rs
```

```
# integration_tests.rs

use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
```

Each file in the `tests` directory is a separate crate, so we need to bring our library into each test crate's scope.
That's why we need to add `use adder;` at the top of `integration_test.rs`.
Cargo treats the `tests` directory specially so we don't need to annotate any code with `#[cfg(test)]`. Rust only 
compiles files in this directory only when we run `cargo test`.

> [!Note]
> If any test in a section fails, the following sections will not be run.

Each integration test file has its own section, so if we add more files in the `tests` directory, there will be more 
integration test sections.

To run all the tests in a particular integration test file, use the `--test` argument of `cargo test` followed by the
name of the file:

```
cargo test --test integration_test
```

### Submodules in Integration Tests

If we create `tests/common.rs` and place a function named `setup` in it, we can add some code to setup that we want
to call from multiple test functions in multiple test files. However, when we run the tests again, we'll see a new 
section in the test output for the `common.rs` file, even though it doesn't contain any test functions.

To avoid having `common` appear in the test output, instead of creating `tests/common.rs` we'll create 
`tests/common/mod.rs`. The project directory now looks like:

```
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── common
    │   └── mod.rs
    └── integration_tests.rs
```

This is the older naming convention. Naming the file this way tells Rust not to treat the `common` module as an 
integration test file. After moving the `setup` code into `tests/common/mod.rs` and delete the old `tests/common.rs`
file, the section in the test output will no longer appear.

Files in subdirectories of the `tests` directory don't get compiled as separate crates or have sections in the 
test output. After creating `tests/common/mod.rs`, we can use it from any of the integration test files as a module.
For example:

```
use adder;
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
```

### Integration Tests for Binary Crates

If we have a binary crate that only contains a `src/main.rs` file and doesn't have a `src/lib.rs` file, we CAN'T create
integration tests in the `tests` directory and bring functions defined in the `src/main.rs` file into scope with a `use`
statement. Only library crates expose functions that other crates can use; binary crates are meant to be run on their own.

This is why we keep a small amount of code in `src/main.rs` that calls logic that lives in the `src/lib.rs` file.
Integration tests can test the library crate with `use` to make the important functionality available.