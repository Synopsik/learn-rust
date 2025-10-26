//---------------------------------------------------------------------------------// 
//                                    Enums                                      //
//---------------------------------------------------------------------------------//

pub(crate)
// Simple Enum
enum IpAddrKind {
    V4,
    V6,
}

pub(crate)
// Enum Values provides an easier and clearer syntax over Structs
enum IpAddrBasicValues {
    V4(String),
    V6(String),
}

pub(crate)
// Can also create complex structures
enum IpAddrComplexValues {
    V4(u8, u8, u8, u8),
    V6(String),
}

pub(crate)
// Structs can also be used as Enum parameter
enum IpAddrEnumStructs {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

//---------------------------------------------------------------------------------// 
//                                    Structs                                      //
//---------------------------------------------------------------------------------//

pub(crate)
struct IpAddr {
    pub(crate) kind: IpAddrKind,
    pub(crate) address: String,
}
// Structs for Enum parameters
pub(crate)
struct Ipv4Addr {
    pub(crate) first: u8,
    pub(crate) second: u8,
    pub(crate) third: u8,
    pub(crate) fourth: u8,
}

pub(crate)
struct Ipv6Addr {
    pub(crate) address: String,
}