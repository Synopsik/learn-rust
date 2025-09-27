pub fn run_all() {
    function_definitions();
    method_definitions();
    implementing_traits();
}

// --------------------------------------------------------------------------------------------- //
//                                  Function Definitions                                         //
// --------------------------------------------------------------------------------------------- //

fn function_definitions() {
    let number_list = vec![34, 50, 25, 100, 65];
    let char_list = vec!['y', 'm', 'a', 'q'];
    
    let result = largest(&number_list);
    println!("The largest number is {result}");
    
    let result = largest(&char_list);
    println!("The largest character is {result}");
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    
    for item in list {
        if item > &largest {
            largest = item;
        }
    }
    largest
}


// --------------------------------------------------------------------------------------------- //
//                                      Method Definitions                                       //
// --------------------------------------------------------------------------------------------- //

fn method_definitions() {
    // Mixup struct using multiple Generic types
    struct Point<X1, Y1> {
        x: X1,
        y: Y1,
    }

    // 
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
    
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y)
}

// --------------------------------------------------------------------------------------------- //
//                                         Traits                                                //
// --------------------------------------------------------------------------------------------- //
pub trait Summary {
    fn summarize(&self) -> String;
}

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

fn implementing_traits() {
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


