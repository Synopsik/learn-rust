# Chapter 6: Enums and Pattern Matching

Enums give you a way of saying a value is one of a possible set of values.
The difference is that structs have all fields valid, while only one variant is valid in an enum.
We can choose any of the enumerated values, but not multiple at the same time.

```
enum IpAddrKind {
    V4,
    V6,
}
```

We can now attach an ***IP Address Kind*** to something.
It can be either V4 or V6 (just not both).
Our enum `IpAddrKind` is a custom data type that we can use anywhere in our code.

```
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

Now both values `IpAddrKind::V4` and `IpAddrKind::V6` are of the same type: `IpAddrKind`

Using this we can create any function that takes any `IpAddrKind`:

```
fn route(ip_kind: IpAddrKind) {}

// This function can be called using any variant
route(IpAddrKind::V4);
route(IpAddrKind::V6);
```

Similar to structs, we can also store data in our enum by attaching the data to the variant of the enum directly.
We also need to specify the data type that is to be stored.

```
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```

Another advantage of enums is that each variant can have different types and amounts of associated data.

We know that version four IP addresses will always have four numeric components that will have values between 0 and 255.
If we wanted to store V4 address as four `u8` values but still express `V6` addresses as one `String` value, 
we wouldn't be able to with a struct.

```
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
```

## Enum Methods

Similar to structs, we use `impl` on our Enum and define our method using self as the first parameter.

```
imple Message {
    fn call(&self) {
        // Method body
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

## Option Enum and Its Advantages Over Null Values

Rust handles null values differently from other languages, there isn't a standard null feature.
Something that may or may not be valid is a common occurrence; we need to handle both cases appropriately.

The `Option<T>` enum is defined in the standard library.
It is used to encode the case where a value may be something or nothing.

```
enum Option<T> {
    None,
    Some(T),
}
```

This enum is included in the prelude, so you don't need to bring it into scope directly.

It's variants are also included in the prelude, `Some(T)` and `None`.

The `<T>` syntax is a feature of Rust that allows the `Some(T)` variant of the `Option` enum to hold 
***one piece of data*** of any type.

This way ***each concrete type*** that takes the place of `T` will make the overall enum `Option<T>` a different type.

```
let some_number = Some(5);
let some_char = Some('e');

let absent_number: Option<i32> = None;
```

The type of some_number is `Option<i32>` and the type of some_char is `Option<char>`

For `None` (null values), Rust requires us to annotate the `Option` type.

The compiler can't infer the type by looking only at a `None` value.
This is why the type of absent_number is specified as `Option<i32>`

### So why is having `Option<T>` any better than having null?

Because, `Option<T>` and `T` are ***different types***!

This means the compiler won't let us use an `Option<T>` value as if it were definitely a valid value.

```
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y; // ERROR!!!
```

Rust doesn't know how to add an `i8` to an `Option<i8>`.
We must first convert the value (if it's valid) to be able to use it.

This paradigm allows us to safely assume that ***EVERY VALUE*** that ***IS NOT*** an `Option<T>`: 
***IS VALID*** and ***NOT NULL***.

## The `match` Control Flow Construct

There are many different methods to do things with `Option<T>` such as extract our value. 

Most of the time, however, we will be using `match` statements to compare the value against a series of patterns.

Patterns can be made up of literal values, variable names, wildcards, and many other things.

Similar to a coin-sorting machine, each coin falls into the first `match`.

```
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

First we list the `match` keyword followed by an expression, which in this case is the value `coin`.
The condition is then evaluated using our enum `Coin`.
Depending on the function input variant, the match has multiple arms that must cover all possibilities.

The arm has two parts: a pattern and some code.

The pattern in our first case is `Coin::Penny` while the code is seperated using `=>`.
If our input pattern is `Coin::Penny` our output code is the `u8` integer `1`.

Each arm is seperated from the next using a comma `,`

If we want to write code that is longer than a single line, we need to enclose the code in curly brackets `{}`.
The comma following the arm is then optional.

```
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

### Matching with `Option<T>`

Let's say we want to write a function that takes an `Option<i32>` and, if there's a value inside, adds 1 to that value.

If there isn't a value inside, the function should return the None value and not attempt to perform any operations.

```
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

Our first variable `Some(5)` attempts to match, this doesn't match `None` so it moves onto the next arm:

```
Some(i) => Some(i + 1),
```

`Some(5)` matches `Some(i)`, `5` is bound to `i`when the code in the match arm is executed and results in a new `Some`
value with our total `6` inside.

In our second call of `plus_one`, the parameter `x` is the value `None`. 
We enter the `match` and compare to the first arm, it matches, so the program stops and returns the `None` value.
Because the fist arm was matched, no other arms are compared.

It's important to remember that `match` patterns MUST cover all possibilities, otherwise you will receive errors.

To cover all other possibilities, use `_` it is a special pattern that matches any values without specifically binding.
`_` also tells Rust we aren't going to use the value, so it will stop warning us about an unused variable.

```
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => (),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
```

We're explicitly ignoring any other value that does not match a pattern in an earlier arm by not running any code.

### Concise Control Flow with `if let`

The `if let` syntax creates a less verbose way to handle values that will only match one pattern and ignore the rest.

```
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {max}");
}
```
This replaces a common situation in a less verbose way.

```
// Old example -- More verbose
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {max}"),
    _ => (),
}
```

`if let` is syntax sugar for a `match` that runs code when the value matches one pattern and ignores all other values.

Also, we can use `else` after the `if let`. 
The code that would go under `_` in our `match` expression would go under `else`.

```
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```