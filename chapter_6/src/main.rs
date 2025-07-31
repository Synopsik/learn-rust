mod enums_and_structs;

use crate::enums_and_structs::{
    IpAddr,
    IpAddrKind,
    IpAddrBasicValues,
    IpAddrEnumStructs,
    IpAddrComplexValues,
    Ipv4Addr,
    Ipv6Addr,
};

fn main() {
    enum_intro();
    enum_basic_values();
    enum_complex_values();
}


fn enum_intro() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // We can use enums as params
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    fn route(ip_kind: IpAddrKind) {} // This could return something

    // We can use Structs that use Enums to create concepts
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}


fn enum_basic_values() {
    // We've condensed and encoded most of the previous code into the enums
    let home = IpAddrBasicValues::V4(String::from("127.0.0.1"));
    let loopback = IpAddrBasicValues::V6(String::from("::1"));
    // This saves space and reads clearer
}

fn enum_complex_values() {
    // We can further condense and itemize our inputs
    let home = IpAddrComplexValues::V4(127, 0, 0, 1);
    let loopback = IpAddrComplexValues::V6(String::from("::1"));
    // This saves us time from converting V4 to u8 later
}

fn enum_struct_values () {
    // We can also use a struct as an input
    let v4_addr = Ipv4Addr {
        first: 0,
        second: 0,
        third: 0,
        fourth: 0,
    };
    // These can be as complex as needed.
    // Struct parameters help make strong types
    let v6_addr = Ipv6Addr {
        address: String::from("::1"),
    };

    let home = IpAddrEnumStructs::V4(v4_addr); // Uses Structs for input
    let loopback = IpAddrEnumStructs::V6(v6_addr); // Must match Struct parameters
}