# Chapter 3: Common Programming concepts

## Notes

The compiler is able to evaluate a limited number of operations (expression) types at runtime.

For example: `const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;`

Even though we are declaring a constant variable, we can do it this way because it's made easier to write
3 hours times 60 minutes times 60 seconds in an expression compared to hardcoded as 10800 for the seconds.

---

We can use Shadowing to "mutate" an immutable variable by calling a new variable to replace it.
In doing this we can change the variable type:
```
let variable = " ";
let variable = variable.len();
```

However, if we try to mutate a variable type with a mutable var, we get an error:
```
let mut variable = " ";
variable = variables.len();
// ERROR!!
```
The error says we cannot mutate the variable type.

---

#### Integer Types in Rust
| Length  | Signed  | Unsigned | Size         |
|---------|---------|----------|--------------|
| 8-bit   | `i8`    | `u8`     | 1 byte       |
| 16-bit  | `i16`   | `u16`    | 2 bytes      |
| 32-bit  | `i32`   | `u32`    | 4 bytes      |
| 64-bit  | `i64`   | `u64`    | 8 bytes      |
| 128-bit | `i128`  | `u128`   | 16 bytes     |
| arch    | `isize` | `usize`  | 4 or 8 bytes |
| 32-bit  | `f32`   | n/a      | 4 bytes      |
| 64-bit  | `f64`   | n/a      | 8 bytes      |

### Formula to find a data types **min to max** size, **-(2<sup>n-1</sup>) to 2<sup>n-1</sup>-1** inclusive.

When we are estimating an **i8** for **n** we result with -(2<sup>**7**</sup>) to 2<sup>**7**</sup>-1 which equals **-128 to 127**.

*Unsigned variants* can store from **0 to 2<sup>**n**</sup>-1** inclusive.

For a u8 we can store from 0 to 2<sup>**8**</sup>-1, which equals **0 to 255**.

`isize` and `usize` depends on the architecture of the computer that the program is running on

#### Other Types in Rust
| Type | Size    |
|------|---------|
| Char | 4 bytes |
| Bool | 1 byte  |

Arrays must have a fixed length when created.
```
let arr: [i32; 5] = [1, 2, 3, 4, 5];  // 20 bytes (5 × 4 bytes)
let bytes: [u8; 10] = [0; 10];        // 10 bytes
```

#### Integer Literals in Rust
| Number literals | Example       | Default Type | Size    |
|-----------------|---------------|--------------|---------|
| Decimal         | `98_222`      | i32          | 4 bytes |
| Hex             | `0xff`        | i32          | 4 bytes |
| Octal           | `0o77`        | i32          | 4 bytes |
| Binary          | `0b1111_0000` | i32          | 4 bytes |
| Byte (u8 only)  | `b'A'`        | u8           | 1 byte  |

#### Heap Allocated
```
let s: String = String::from("hello");  // 24 bytes on stack (ptr + len + capacity)
                                        // + heap allocation for actual string data
let str_ref: &str = "hello";            // 16 bytes (pointer + length)
```

### Pointer Sizes

#### Regular Reference
```
let x = 42;
let r: &i32 = &x;      // 8 bytes on 64-bit systems (just a pointer)
let mr: &mut i32;      // 8 bytes on 64-bit systems
```

#### Fat Reference (DST Reference)
```
let s: &str = "hello";           // 16 bytes (8-byte pointer + 8-byte length)
let slice: &[i32] = &[1, 2, 3];  // 16 bytes (8-byte pointer + 8-byte length)
let obj: &dyn Display = &42;     // 16 bytes (8-byte pointer + 8-byte vtable pointer)
```


---

### Evaluating Expressions

Expressions alone do not need to include ending semicolons (`;`),
```
let y = {
    let x = 3; // Statement, semicolon
    x + 1 // Expression, doesn't require a semicolon
}; // Main Statement, semicolon
```
it's only when paired with a statement that you need to close with a semicolon:
`let x = x + 1;`

---

### Loop Labels

Loop labels can be used to break a specific loop at any point in the nested hierarchy.
```
'outer_loop: loop {
    `inner_loop: loop {
        break `outer_loop;
    }
}
```

While loops can be slower than for loops because of the additional check for validity during each iteration

Using a For loop instead, we can increase the safety of the code by removing the chance for the index to either
go beyond the end of the array or not go far enough and miss some items.
```
let a = [10, 20, 30, 40, 50]

for element in a {
    println!("{element}");
}
```

Therefore, ONLY use while loop you are absolutely uncertain how many iterations are needed
