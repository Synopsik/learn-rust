# Chapter 9: Error Handling

Rust groups errors into two major categories: ***recoverable*** and ***unrecoverable*** errors.

## Recoverable Errors

Uses the type `Result<T, E>` 

For a recoverable error like a `file not found` error, we may just want to report the problem to the user and retry. 

The `Result` enum is defined as having two variants, `OK` and `Err`:
```
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
The `Result` enum uses generic type parameters
* `T`: Represents the type of value that will be returned in a success case within the `Ok` variant
* `E`: Represents the type of error that will be returned in a failure case within the `Err` variant

We can use the `Result` type and the functions defined on it in many different situations where the *success value* and 
*error value* we want to return may differ. Like the `Option` enum, the `Result` enum and its variants have been
brought into scope by the prelude, so we don't need to specify `Result::` before the `Ok` and `Err` variants in the 
`match` arms.

## Unrecoverable Errors

Uses the `panic!` macro.

These are always symptoms of bugs, such as trying to access a location beyond the end of an array,
so we immediately stop the program.

Two ways to cause a panic: 

1. Taking action that causes our code to panic 
2. By explicitly calling the `panic!` macro.

By default, this will print a failure message, unwind, clean up the stack, and quit.

---

We can set the `RUST_BACKTRACE` environment variable to any value except `0` to get a backtrace of exactly what 
happened to cause the error. To get backtraces with this information, debug symbols must be enabled.
***Debug Symbols*** are enabled by default when using `cargo build` or `cargo run` without the `--release` flag.

## Matching on Different Errors

`File::open` can fail for various reasons, for example, because we don't have permission to open the file.
We can add an inner `match` expression to catch multiple types of errors:
```
use std::fs::File;
use std::io::ErrorKind;
...
let greeting_file_result = File::open("hello.txt");

let greeting_file = match greeting_file_result {
    Ok(file) => file,
    Err(error) => match error.kind() {
        ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!(
                    "Problem creating the file: {:?}", e
                ),
            }
        }
        other_error => {
            panic!(
                "Problem opening the file: {:?}", other_error
            );
        }
    },
};
...
```

The `Err` variant `io::Error` is a struct provided by the standard library. This struct has a method `kind` that we can
call to get an `io::ErrorKind` value. The enum `io::ErrorKind` is provided by the standard library and has variants
representing the different kinds of errors that might result from an `io` operation.

The condition we want to check in the inner match is whether the value returned by `error.kind()` is the `NotFound`
variant of the `ErrorKind` enum. 

If it is, we can create a new file with `File::create`. However, because this could
also fail, we need a second arm in the inner `match` expression. If the file can't be created, a different error 
message is printed. 

### Shorcuts for Panic on Error: `unwrap` and `expect`

The `Result<T, E>` type has many helper methods defined on it to do various, more specific tasks.

The `unwrap` method is a shortcut method implemented just like the `match` expression.
If the `Result` value is the `Ok` variant, `unwrap` will return the value inside the `Ok`.
If the `Rsult` is the `Err` variant, `unwrap` will call the `panic!` macro.

Similarly, the `expect` method lets us also write the `panic!` error message.

In production-quality code, most choose `expect` rather than `unwrap` and give more context about why the operation is
expected to always succeed. That way, if your assumptions are ever proven wrong, you have more information to use in 
debugging.

## Propagating Errors

Instead of handling an error within a function where it fails, you can return the error to the calling code so that it
can decide what to do. This is known as ***propagating*** the error and gives more control to the calling code, where
there might be more information or logic to handling complex errors.

```
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    
    let mut username = String::new();
    
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
    
}
```

The return type of the function is `Rsult<String, io::Error>`. 
Meaning we have defined our function to return the type `Result<T, E>`, where the generic parameter `T` has been filled
in with the concrete type `String` and the generic type `E` has filled in with the concrete type `io::Error`.

If this function succeeds, the code that calls this function will receive an `Ok` value that holds a `String` value for
the `username` that is read from the file. 

If this function encounters problems, the calling code will receive an `Err` value that holds an instance of `io::Error`
that contains more information about what the problems were.

## The `?` Operator

The `?` placed after a `Result` value is defined to work in almost the same way as `match` expressions.

If the value of the `Result` is an `Ok`, the value inside the `Ok` will get returned from this expression.

If the value is an `Err`, the `Err` will be returned from the whole function as if we had used the `return` keyword so 
the error value gets *propagated* to the calling code.

You're only allowed to use the `?` operator in a function that returns `Result`, `Option`, or another type that
implements `FromResidual`. To fix this error you have two choices:

* Change the return type of you function to be compatible with the value you're using the `?` operator on (as long as
  you have no restrictions preventing that)
* Use a `match` or one of the `Result<T, E>` methods to handle the `Rsult<T, E>` in whatever way is appropriate.

---

```
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
```

This function returns `Option<char>` because it's possible there may or may not be a char. We take the `text` string 
slice and call the lines method to create an iterator over the lines in the string. Because this function wants to 
examine the first line, it calls `next` on the iterator to get the first value from the iterator. If `text` is the empty 
string, this call to `next` will return `None`, in which case we use ? to stop and return `None` from our func. If 
`text` is not the empty string `next` will return a `Some` value containing a string slice of the first line in `text`.