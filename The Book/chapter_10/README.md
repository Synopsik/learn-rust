# Chapter 10: Generic Types, Traits, and Lifetimes

## Generic Types

### In Function Definitions

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

### In Struct Definitions

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

### In Enum Definitions

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

### In Method Definitions

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

### Performance of Code Using Generics

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

After `impl`, we put the trait name we want to implement, then use the `for` keyword, and then specify the name of the 
type we want to implement the trait for. Within the `impl` block, we put the method signatures that the trait 
definition had defined previously.

Now that the library has implemented the `Summary` trait on `NewsArticle` and `Tweet`, users of the crate can call the 
trait methods on instances of `NewsArticle` and `Tweet` in the same way we call regular methods. The only difference
is that the user must bring the trait into scope as well as the types.

```
fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };
        
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL",
        ),
    };
    
    println!("1 new tweet: {}", tweet.summarize());
    println!("New article available! {}", article.summarize());
}
```

Other crates that depend on the `aggregator` create can also bring the `Summary` trait into scope to implement `Summary`
on their own types.

```
use aggregator::{Summary, Tweet};
```

> [!Note]
> One restriction to note is that we can implement a trait on a type only if either the trait or the type, or both, are
> local to our crate. For example, we can implement standard library traits like `Display` on a custom type like `Tweet`
> as part of our `aggregator` create functionality because the type `Tweet` is local to our `aggregator` crate. We can
> also implement `Summary` on `Vec<T>` in our `aggregator` create because the trait `Summary` is local to our
> aggregator crate.

But we can't implement external traits on external types. For example, we can't implement the `Display` trait on 
`Vec<T>` within our `aggregator` create because `Display` and `Vec<T>` are both defined in the standard library and 
aren't local to our aggregator crate.

This restriction is part of a property called **coherence**, and more specifically the ***orphan rule***, so named
because the parent type is not present. This rule ensures that other people's code can't break your code and vice
versa. Without the rule, two crates could implement the same trait for the same type, and Rust wouldn't know which
implementation to use.

### Default Implementations

Sometimes we may want to have default behavior for some or all of the methods in a trait instead of requiring
implementations for all methods on every type. Then, after we implement the trait on a particular type, 
we can keep or override each method's default behavior.

```
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)");
    }
}
```

Even though we're no longer defining the `summarize` method on `NewsArticle` directly, we've provided a **default 
implementation** and specified that `NewsArticle` implements the `Summary` trait. 

```
let article = NewsArticle {
    headline: String::from("Penguins win the Stanley Cup Championship!"),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from(
        "The Pittsburgh Penguins once again are the best hockey team in the NHL."
    ),
};

println!("New article available! {}", article.summarize());

// Prints: New article available! (Read more...)
```

Default implementations can call other methods in the same trait, even if those other methods don't have a default 
implementation. A trait can provide a lot of useful functionality and only require implementations to specify a small 
part of it. For example, we could define the `Summary` trait to have a `summarize_author` method whose implementation
in required, and then define a `summarize` method that has a default implementation that calls the `summarize_author`
method.

```
pub trait Summary {
    fn summarize_author(&self) -> String;
    
    fn summarize(&self) -> String {
        format!(
            "(Read more from {}...)",
            self.summarize_author()
        )
    }
}
```

To use this version of `Sumamry`, we only need to define `summarize_author` when we implement the trait on a type.

```
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
```

Because we've implemented `summarize_author`, the `Summary` trait has given us the behavior of the `summarize` method
without requiring us to write any more code.

### Traits as Parameters

We can use traits to define functions that accept many different types. We'll use the `Summary` trait we implemented on 
the `NewsArticle` and `Tweet` types previously to define a notify function that calls the `summarize` method on its 
`item` parameter, which is of some types that implements the `Summary` trait.

```
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

We specify the `impl` keyword and the trait name. This parameter accepts any type that implements the specified trait.
In the body of `notify`, we can call any methods on `item` that come from the `Summary` trait, such as `summarize`.
Code that calls the function with any other type, such as a `String` or an `i32`, won't compile because those types 
don't implement `Summary`.

### Trait Bound Syntax

The `impl` Trait is appropriate if we want this function to allow `item1` and `item2` to have different types. But if
want to force both parameters to have the same type, we must use a ***trait bound***:

```
pub fn notify<T: Summary>(item1: &T, item2: &T) {
    ...
}
```

The generic type `T` specified as the type of the `item1` and `item2` parameters constraints the function such that the
concrete type of the value passed must be the same.

### Multiple Trait Bound Using + Syntax

We can also chain more than one trait bound. Say we wanted `notify` to use display formatting as well as `summarize`
on `item`. We specify in the `notify` definition that `item` must implement both `Display` and `Summary`. We can do 
this using the `+` syntax:

```
fn notify(item: &(impl Summary + Display)) { ... }

// The + syntax is also valid with trait bound

fn notify<T: Summary + Display>(item: &T) { ... }
```

The body of `notify` can now call `summarize` and use `{}` to format items.

### `where` Clause

Rust has an alternate syntax for when we begin to specify too many trait bounds:

```
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    ...
}
```

This makes our function's signature less cluttered and clear to read.

### Returning Types that Implement Traits

We can use the `impl Trait` syntax in the return position to return a value of some type that must implement a trait:

```
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people",
        reply: false,
        retweet: false,
    }
}
```

We are specifying that the function returns some type that implements the `Summary` trait without naming the concrete 
type. The function actually returns a `Tweet`, but the code calling this function doesn't need to know that.

However, you can only use `impl Trait` if you're returning a single type. For example, this code that could potentially
return either a `NewsArticle` or a `Tweet` with the return type specified as `impl Summary` would work:

```
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle { ... }
    } else {
        Tweet { ... }
    }
}
/* !!!ERROR!!! */
```

### Trait Bounds to Conditionally Implement Methods
By using a trait bound with an `impl` block that uses generic type parameters, we can implement methods conditionally 
for types that implement the specified traits. 

In the example below, the type `Pair<T>` always implements the `new` function to return a new instance of `Pair<T>`. 

But in the next `impl` block, `Pair<T>` only implements the `cmp_display` method if its inner type `T` implements the
`PartialOrd` trait that enables comparison *and* the `Display` trait that enables printing.

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

We can also conditionally implement a trait for any type that implements another trait.
Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations.

For example, the standard library implements the `ToString` trait on any type that implements the `Display` trait.
Because of this blanket implementation, we can call the `to_string` method defined by the `ToString` trait on any type
that implements the `Display` trait.

```
// Turn integers into their corresponding `String` values, because integers implement `Display`

let s = 3.to_string();
```

### Validating References with Lifetimes

_Lifetimes_ ensure that **references** are valid as long as we need them to be. 
Every **reference** in Rust has a _lifetime_, which is the scope for which that **reference** is valid. Usually, _lifetimes_ are 
**implicit and inferred**, just like how types are usually inferred. 
We need to annotate _lifetimes_ when the _lifetimes_ of **references** could be related in a few different ways.

Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used
at runtime will definitely be valid.
The aim of lifetimes is to prevent _dangling references_, which causes a program to reference invalid data.

```
// Dangling Reference Example
fn main() {
    let r;
    {
        let x = 5;
        r = &x;
    }
    
    println!("r: {r}"); // Error! `r` references a value that is no longer in scope
}
```

> [!WARNING]
> This code won't compile because the value that `r` is referring to has gone out of scope before we try to use it. 
>
> Inside the inner scope, we attempt to set the value of `r` as a reference to `x`.
> Then the inner scope ends, and we attempt to print the value in `r`. 

Because the scope of `r` is larger, we say that it "lives longer."

### Generic Lifetimes in Functions

Let's demonstrate the issue before the solution by creating a function that returns the longer of two string slices.
```
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}
```

Notice the function is taking string slices, which are references, instead of strings, this is because we don't
want the `longest` function to take ownership of its parameters. If we try to implement the `longest` function as shown
below, it won't compile.

```
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

> [!WARNING]
> The function return type needs a generic lifetime parameter on it because Rust can't tell whether the reference being 
> returned refers to `x` or `y`.

We don't know either, because the inputs `x` and `y` decide which path the `if` statement
will take. We also don't know the concrete lifetimes of all the references that might be passed in, we can't look at the 
scope of each caller to determine whether the reference we return will always be valid. 

The borrow checker can't determine this either, because it doesn't know how the lifetimes of `x` and `y` relate to the
lifetime of the return value. To fix this error, we'll add **generic lifetime parameters** that define the relationship 
between the references so the borrow checker can perform its analysis.

### Lifetime Annotation Syntax

Lifetime annotations don't change how long any of the references live. They describe the relationships of the lifetimes 
of multiple references to each other without affecting the lifetimes.

Just as functions can accept any type when the signature specifies a generic type parameter, functions can accept 
references with any lifetime by specifying a generic lifetime parameter. 

The names of lifetime parameters must start with an apostrophe `'` and are usually all lowercase and very short,
like generic types. Most people use the name `'a` for the first lifetime annotation. We place lifetime parameter 
annotations after the `&` of a reference, using a space to separate the annotation from the reference's type.

```
&i32            // a reference
&'a i32         // a reference with an explicit lifetime
&'a mut i32     // a mutual reference with an explicit lifetime
```

One lifetime annotation by itself has little meaning because the annotations are meant to tell Rust how generic 
lifetime parameters of multiple references relate to each other. To use lifetime annotations in function signatures,
we need to declare the generic _lifetime_ parameters inside angle brackets between the function name and the parameter 
list, just as we did with generic _type_ parameters.

```
fn longest<`a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

This code will now compile and produce the expected result. The signature expresses the following constraint: 
the returned reference will be valid as long as both the parameters are valid. 
This is the relationship between lifetimes of the parameters and the return value. 
We name the lifetime `'a` and add it to each reference.

In practice, this means the lifetime of the reference returned by the `longest` function is the same as the smaller of
the lifetimes of the values referred to by the function arguments. Remember, when we specify the lifetime parameters in 
this function signature, we're not changing the lifetimes of any values passed in or returned. We're just specifying
that the borrow checker should reject ay values that don't adhere to these constraints.

> [!Note]
> The `longest` function doesn't need to know exactly how long `x` and `y` will live, only that some scope can be
> substituted for `'a` that will satisfy this signature.
 
The annotations go in the function signature, not in the function body. The lifetime annotations become part of the 
contract of the function, much like the types in the signature. This allows compiler errors to point to part of our 
code and the constraints more precisely. 

When we pass concrete references to `longest`, the concrete lifetime that is substituted for `'a` is the part of the 
scope of `x` that overlaps with the scope of `y`. In other words, the generic lifetime `'a` will get the concrete 
lifetime that is equal to the smaller of the lifetimes of `x` and `y`.

Because we've annotated the returned reference with the same lifetime parameter `'a`, the returned reference will also 
be valid for the length of the smaller of the lifetimes of `x` and `y`.

The way in which you need to specify lifetime parameters depends on what your function is doing. For example, if we 
changed the implementation of the `longest` function to always return the first parameter rather than the longest 
string slice, we wouldn't need to specify a lifetime on the `y` parameter.

```
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime
parameter for one of the parameters. If the reference returned does not refer to one of the parameters, it must refer
to a value created within this function. However, this would be a dangling reference because the value will go out of 
scope at the end of the function.

```
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
```
> [!WARNING]
> Even though we've specified a lifetime parameter `'a` for the return type, this implementation will fail to compile 
> because the return value lifetime is not related to the lifetime of the parameters at all.
 
The problem is that `result` goes out of scope and gets cleaned up at the end of the `longest` function. There is no 
way we can specify lifetime parameters that would change the dangling reference, Rust won't allow it. The best fix is to 
return an owned data type rather than a reference so that calling function is then responsible for cleaning up the value.

### Lifetime Annotations in Struct Definitions

So far we've only created structs that held owned types, if we want to hold a reference in a struct, we need to add a 
lifetime annotation on every reference in the struct's definition.

```
struct ImportExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    
    let first_sentence = novel
        .split('.')
        .next()
        .expect("Could not find a '.'");
        
    let i = ImportExcerpt { 
        part: first_sentence, 
    );
}
```

As with generic data types, we declare the name of the generic lifetime parameter inside angle brackets after the name 
of the struct so we can use the lifetime parameter in the body of the struct definition. This annotation means an
instance of `ImportantExcerpt` can't outlive the reference it holds in its `part` field. `novel` doesn't go out of scope
until after the `ImportantExcerpt` goes out of scope, so the reference in the `ImportantExcerpt` instance is valid.

### Lifetime Elision

We've learned that every reference has a lifetime and that you need to specify lifetime parameters for functions or
structs that use references. However, let's look at a function that uses references and compiles without lifetime 
annotations.

```
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}
```

Historically, this would need explicit lifetime annotations, but after writing lots of code the Rust team found they 
were entering the same lifetime annotations over and over in particular deterministic situations. 
The developers programmed these patterns into the compiler's code so the borrow checker could infer the lifetimes in 
these situations and wouldn't need explicit annotations.

These are called the _lifetime elision rules_, they are a set of particular cases that the compiler will consider,
and if your code fits these cases, you don't need to write the lifetimes explicitly. 
If Rust deterministically applies the rules but there is still ambiguity as to what lifetimes the references have,
the compiler won't guess what the lifetime of the remaining references should be. Instead, the compiler will give you
an error that you can resolve by adding the lifetime annotations.

Lifetimes on function or method parameters are called _input lifetimes_, 
and lifetimes on return values are called _output lifetimes_.
The compiler uses three rules to figure out the lifetimes without explicit annotations. The first rule applies to input
lifetimes, and the second and third rules apply to output lifetimes. If the complier gets to the end of the three rules and
there are still references with lifetimes it can't determine, the compiler will stop with an error.
These rules apply to `fn` definitions as well as `impl` blocks.

### Lifetime Elision Rules

#### Rule 1
The compiler assigns a lifetime parameter to each parameter that's a reference. 
For example: 
* A function with one parameter gets one lifetime parameter: `fn foo<'a>(x: &'a i32) { ... }`
* A function with two parameters gets two separate lifetime parameters: `fn foo<'a, 'b>(x: &'a i32, y: &'b i32) { ... }`
* And so on...

#### Rule 2
If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters:
`fn foo<'a>(x: &'a i32) -> &'a i32 { ... )`

#### Rule 3
If there are multiple input lifetime parameters, but one of them is `&self` or `&mut self` because this is a method, the 
lifetime of self is assigned to all output lifetime parameters. This rules makes methods nicer to read and write since fewer
 symbols are necessary.

We can apply these rules to better understand how the compiler applies them.

#### Example #1

```
fn first_word(s: str) -> &str { ... }
```

For the first rule, the compiler gives all parameters its own lifetime.

```
fn first_word<'a>(s: &'a str) -> &str { ... }
```

The second rule applies because there is exactly one input lifetime. This will specify that the lifetime of the one input
parameter gets assigned to the output lifetime.

```
fn first_word<'a>(s: &'a str) -> &'a str { ... }
```

Now all the references in this function signature have lifetimes, and the compiler can continue its analysis without needing 
the programmer to annotate the lifetimes in this function signature.

#### Example #2

Let's look at the `longest` function that failed earlier because it had no lifetime parameters.

```
fn longest(x: &str, y: &str) -> &str { ... }
```

For the first rule, each parameter gets its own lifetime.

```
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str { ... }
```

The second rule doesn't apply because there is more than one input lifetime.

The third rule also doesn't apply, because `longest` is a function rather than a method, so none of its parameters are `self`.

After working through all three rules, we still don't know what the return type's lifetime is.
The compiler worked through the lifetime elision rules but still couldn't figure out all the lifetimes of the references in
the signature.

#### Example #3

When we implement methods on a struct with lifetimes, we use the same syntax as that of generic type parameters.
Where we declare and use the lifetime parameters depends on whether they're related to the struct fields _or_ the method 
parameters and return values.

Lifetime names for struct fields always need to be declared after the `impl` keyword and then used after the struct's name
because those lifetimes are part of the struct's type. 

```
impl <'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```

The lifetime parameter declaration after `impl` and its use after the type name are required, but we're not required to
annotate the lifetime of the reference to `self` because of the first elision rule.

#### Example 4

In this example, there are two input lifetimes, so Rust applies the first lifetime elision rule and gives both `&self` and 
`announcement` their own lifetimes. Then, because one of the parameters is `&self`, the return type gets the lifetime of
`&self`, and all lifetimes have been accounted for.

```
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}
```

### The Static Lifetime

`'static` denotes that the affected reference _can_ live for the entire duration of the program.
All string literals have the `'static` lifetime, which we can annotate as follows:

```
let s: &'static str = "I have a static lifetime.";
```

The text of this string is stored directly in the program's binary, which is always available.
Therefore, the lifetime of all string literals is `'static`.

### All Together Now

Now this function has an extra parameter named `ann` of the generic type `T`, which can be filled in by any type that 
implements the `Display` trait as specified by the `where` clause. 

This extra parameter will be printed using `{}`, which is why the `Display` trait bound is necessary. Because lifetimes are
a type of generic, the declarations of the lifetime parameter `'a` and the generic type parameter `T` go in the same list 
inside the angle brackets after the function name.

```
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```