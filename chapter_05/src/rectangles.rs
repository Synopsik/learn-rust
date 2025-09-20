fn main() {
    first_example();
    tuple_example();
    struct_example();
    struct_methods_example();
}

fn first_example() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
    fn area(width: u32, height: u32) -> u32 { width * height }
    // Regular way to calculate the area with two separate variables
}

fn tuple_example() {
    let rect1 = (30, 50);
    // Using a tuple, you can condense the two W*H variables into a single rect1

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
    fn area(size: (u32, u32)) -> u32 { size.0 * size.1 }
    // Handling multiple indexes as parameters can get confusing and lead to errors
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        // A struct method that calculates area for a Rectangle
        self.width * self.height
    }

    fn width(&self) -> bool {
        // Determines if width is greater than 0
        self.width > 0
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        // Checks height AND width to determines sizes
        rect.height < self.height && rect.width < self.width
    }
}

fn struct_example() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    fn area(rectangle: &Rectangle) -> u32 { rectangle.width * rectangle.height }
    // Now we can call the individual traits instead of an ambiguous index

    println!("rect1 is {:?}", rect1); // Debug enabled to print structs
    println!("rect1 is {:#?}", rect1); // Nicer formatting

    // Struct methods
    println!("The area of the rectangle is: {}", rect1.area());

    if rect1.width() {
        println!(
            "The rectangle has a nonzero width; it is {}",
            rect1.width(),
        );
    }
}

fn struct_methods_example() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

}

