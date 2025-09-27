# Chapter 10: Generic Types, Traits, and Lifetimes

## In Function Definitions
Generics allow us to replace specific types with a placeholder that represents multiple types to remove code 
duplication. We use generics to create definitions for items like functions, signatures, or structs, which we can then
use with many different concrete data types.

When defining a function that uses generics, we place the generics in the signature of the function where we would 
usually specify the data types of the parameters and return value. You can use any identifier as a type parameter name.
But we'll use `T` because, by convention, type parameter names in Rust are short, often just one letter, and Rust's 
type-naming convention is UpperCamelCase. 

Short for *type*, `T` is the default choice for most Rust programmers.

```
fn largest<T>(list: &[T]) -> &T {
...
```

This reads as: **The function `largest` is generic over some type `T`**

We have to declare the parameter name in the signature before we can use a parameter in the body of the function.
Similarly, when we use a type parameter name in a function signature, we have to declare the type parameter name before
we use it. To define the generic `largest` function, we place type name declarations inside angle brackets, `<>`,
between the name of the function and the parameter list. When using generics, sometimes you'll need to perform an 
operation that may be valid for some types, but invalid for others. We can use ***traits*** to implement on generic 
types to restrict what's allowed.

For example, to enable comparing numbers using `<`,`>`, or `==` we need to implement the `std::cmp::PartialOrd` trait 
on our type. This restricts the types valid for `T` to only those that implement `PartialOrd`.

```
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    
    for item in list {
        if item > &largest {
            largest = item;
        }
    }
    largest
}
```

## In Struct Definitions

We can also define structs to use a generic type parameter in one or more fields using the `<>` syntax.

```
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```

First declare the name of the type parameter inside angle brackets just after the name of the struct. Then we use the 
generic type in the struct definitions where we would otherwise specify concrete data types. Fields `x` and `y` are
both the same type.

To define a `Point` struct where `x` and `y` are both generics but could have different types, we can use multiple 
generic type parameters.

```
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
```

You can use as many generic type parameters in a definition as you want, 
but using more than a few makes your code hard to read.

## In Enum Definitions

We can define enums to hold generic data types in their variants.

```
enum Option<T> {
    Some(T),
    None,
}
```

The `Option<T>` enum is generic over type `T` and has two variants: 
* `Some`: Holds one value of type `T`
* `None`: Doesn't hold any value

Enums can have multiple generic types as well.

```
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

The `Result` enum is generic over two types, `T` and `E`, and has two variants:
* `Ok`: Holds a value of type `T`
* `Err`: Holds a value of type `E`

This makes it convenient to use the `Result` enum anywhere we have an operation that might succeed.

## In Method Definitions

In the same way we can implement methods on structs and enums we can use generic types in their definitions too.
The code below shows a method named `x` implemented on the `Point<T>` struct.

```
struct Point<T> { x: T, y: T, }

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
}
```

Methods written within an `impl` that declares the generic types will be defined on any instance of the type,
no matter what concrete type ends up substituting for the generic type.

We can also specify constraints on generic types when defining methods on the type.
For example, we could implement methods only on `Point<f32>` instances rather than on `Point<>` instances with any 
generic type. 

Other instances of `Point<T>` where `T` is not of type `f32` will not have this method defined.

```
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

Some generic parameters can be declared with `impl` and some are declared with the method definitions. In the example 
below, the generic parameters `X1` and `Y1` are declared after `impl` because they go with the struct definition.
The generic parameters `X2` and `Y2` are declared after `fn mixup` because they're only relevant to the method.
 
```
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(
        self,
        other: Point<X2, Y2>,
    ) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    
    let p3 = p1.mixup(p2);
    
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

## Performance of Code Using Generics

Generic types won't make your program run any slower than it would with concrete types.

Rust accomplishes this by performing ***Monomorphization***, which is the process of turning generic code into specific 
code by filling in the concrete types that are used when compiled. 

In this process, the compiler does the opposite of the steps we used to create the previous generic functions:
* The compiler looks at all the places where genric code is called
* generates code for the concrete types the generic code is called with

```
let integer = Some(5);
let float = Some(5.0);
```

When Rust compiles this code, it performs ***monomorphization***. The compiler reads the values that have been used in
`Option<T>` instances and identifies two kinds of `Option<T>`:
* `Some(5)` = `i32`
* `Some(5.0)` = `f64`

It expands the generic definition of `Option<T>` into two definitions specialized to `i32` and `f64`, thereby replacing
the genric definitions with the specific ones.

## Traits: Defining Shared Behavior

### Defining a Trait

A ***trait*** defines the functionality a particular type has and can share with other types.
We can use ***trait bounds*** to specify that a generic type can be any type that has certain behavior.

> [!Note]
> Traits are similar to a feature often called **interfaces** in other languages, although with some differences.

***Trait Definitions*** are a way to group method signatures together to define a set of behaviors necessary to 
accomplish some purpose.

For example, let's say we have multiple structs that hold various kinds and amounts of text:
* A `NewsArticle` struct that holds a news story filed in a particular location
* A `Tweet` that can have, at most, 280 characters along with metadata

We want to make a media aggregator lib crate name `aggregator` that can display summaries of data that might be stored 
in a `NewsArticle` or `Tweet` instance. We now need a summary from each type, and we'll request that summary by calling
a `summarize` method on an instance.

```
pub trait Summary {
    fn summarize(&self) -> String;
}
```

We've declared a trait using the `trait` keyword and then the trait's name, which is `Summary` in this case.
We also need to declare the trait as `pub` so that crates depending on this crate can make use of this trait too.
Inside the curly brackets, we declare the method signatures that describe the behaviors of the types that implement 
this trait, which in this case is `fn summarize(&self) -> String`.

After the method signature, instead of providing an implementation within curly brackets, we use a semicolon.
Each type implementing this trait must provide its own custom behavior for the body of the method.
The compiler will enforce that any type that has the `Summary` trait must have the method `summarize` defined with this
signature exactly.

### Implementing a Trait on a Type

Let's show an implementation of the `Summary` trait on the `NewsArticle` struct that uses the headline, the author, and
the location to create the return value of `summarize`. For the `Tweet` struct, we define `summarize` as the username 
followed by the entire text of the tweet. 

```
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(
            "{}, by {} ({})",
            self.headline,
            self.author,
            self.location
        )
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

