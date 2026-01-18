# Chapter 13: Functional Language Features: Iterators and Closures

Rust's closures are anonymous functions you can save in a variable or pass as arguments to other functions.

You can create the closure in one place and then call the closure elsewhere to evaluate it in a different context.

Unlike functions, closures can capture values from the scope in which they're defined.

## Closures

### Capturing the Environment with Closures

```
|| self.most_stocked()
```

This is a closure that takes no parameters. The body of the closure calls `self.most_stocked()`.

The closure captures an immutable reference to the `self` *Inventory* instance and passes it with the code we specify
to the `unwrap_or_else` method. Functions are not able to capture their environment in this way.

Closures also don't usually require you to annotate the types of the parameters or the return value.
We can add type annotations if we want to increase explicitness and clarity at the cost of being more verbose.

```
let expensive_closure = |num: u32| -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
```

Shown below are the differences between functions, closures w/ annotations, and closures w/o annotations.

```
fn add_one_v1 (x: u32) -> u32 { x + 1 }

let add_one_v2 = |x: u32| -> u32 { x + 1 };

let add_one_v3 = |x| { x + 1 };

let add_one_v4 = |x| x + 1;
```

The compiler will infer one concrete type for each of their parameters and for their return value. 
Because there are no type annotations, we can call the closure with any type. However, if we then try to call the
closure with a different type, an error will occur.

```
let example_closure = |x| x;

let s = example_closure(String::from("hello"));
let n = example_closure(5); // Error!!!
```

### Capturing References or Moving Ownership

Rust's closures are anonymous functions you can save in a variable or pass as arguments to other functions.

You can create the closure in one place and then call the closure elsewhere to evaluate it in a different context.

Unlike functions, closures can capture values from the scope in which they're defined.

## Capturing the Environment with Closures

```
|| self.most_stocked()
```

This is a closure that takes no parameters. The body of the closure calls `self.most_stocked()`.

The closure captures an immutable reference to the `self` *Inventory* instance and passes it with the code we specify
to the `unwrap_or_else` method. Functions are not able to capture their environment in this way.

Closures also don't usually require you to annotate the types of the parameters or the return value.
We can add type annotations if we want to increase explicitness and clarity at the cost of being more verbose.

```
let expensive_closure = |num: u32| -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
```

Shown below are the differences between functions, closures w/ annotations, and closures w/o annotations.

```
fn add_one_v1 (x: u32) -> u32 { x + 1 }

let add_one_v2 = |x: u32| -> u32 { x + 1 };

let add_one_v3 = |x| { x + 1 };

let add_one_v4 = |x| x + 1;
```


