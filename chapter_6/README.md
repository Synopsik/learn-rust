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