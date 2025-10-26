fn main() {
    let m = Message::Write(String::from("hello"));
    m.test_call();
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn test_call(&self) {
        match self {
            Message::Quit => println!("Quitting the application"),
            Message::Move {x, y} => println!("Moving to coordinates: (X:{} Y:{})", x, y),
            Message::Write(text) => println!("Writing message: {text}"),
            Message::ChangeColor(r, g, b) => println!("Changing color to R:{} G:{} B:{}", r, g, b),

        }
    }
}