# Chapter 10: Generic Types, Traits, and Lifetimes

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

We have to declare the parameter name in the signature before we can use a parameter in the body of the function.
Similarly, when we use a type parameter name in a function signature, we have to declare the type parameter name before
we use it. To define the generic `largest` function, we place type name declarations inside angle brackets, `<>`,
between the name of the function and the parameter list.

This reads as: the function `largest` is generic over some type `T`


