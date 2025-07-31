mod rectangles;

fn main() {
    let user1 = defining_structs();
    let user2 = struct_update_syntax(user1);
    tuple_struct();
}


struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


// Tuple Structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit Struct
struct AlwaysEqual;

fn tuple_struct() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("Color: {}\nOrigin: {}", black.0, origin.0);

    let subject = AlwaysEqual; // Unit Struct Init
}


fn create_user(username: String, email: String) -> User {
    /*
        Function that returns User, using username and email parameters
     */
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn struct_update_syntax(user1: User) -> User {
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 /* Struct update syntax
        Or you can also update values individually...
        email: user.email
        */
    };
    user2
}

fn defining_structs() -> User {
    let mut user1 = create_user(
        "Synopsik".parse().unwrap(),
        "synopsik@proton.me".parse().unwrap(),
    );

    let _x = 1;

    println!("{}\n{}\n{}\n{}",
             user1.active,
             user1.username,
             user1.email,
             user1.sign_in_count,
    );

    user1.email = String::from("anotheremail@example.com");

    println!("{}", user1.email);

    user1
}



