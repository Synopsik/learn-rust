# Chapter 4: Understanding Ownership

## Notes

#### Rust's three main rules of ownership

1. Each **value** in Rust has an ***owner***
2. There can only be **one** ***owner*** at a time
3. When the ***owner*** goes out of scope, the **value** will be dropped

---

`String` allocates memory on the heap instead of on the stack and is ***mutable***

The *double colon* operator `::` allows us to namespace this particular `from` function under the `String` type
```
let s = String::from("Hello World!");
```


String literals, on the other hand, are allocated on the stack.
They are fast and efficient because we know their size at compile time.
A string literal is hardcoded in the final executable and is ***immutable***.
```
let s = "Hello World!";
```

---

### Memory and Allocation


With a `String` type, the value is ***mutable*** and growable.
Therefore, we need to allocate an amount of memory on to the heap.

Using memory allocator requires two steps:
1. Memory must be ***requested*** from the **memory allocator** at runtime
2. When we are done with this memory, we need to ***return*** it to the allocator

In Rust, we don't need to worry about garbage collection;
when a variable goes out of scope, the memory is automatically returned.

