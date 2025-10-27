use std::fmt::{Debug, Display};
use std::iter::Sum;

pub fn run_traits() {
    implementing_traits();
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
        author: String::from("Iceberg"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL",
        ),
    };

    println!("1 new tweet: {}", tweet.summarize()); // Custom summarize method for Tweets
    println!("New article available! {}", article.summarize()); // And Articles

    notify(&tweet); // Can only call this on objects that implement the Summarize trait
    notify_expanded(&tweet, &tweet);
}


pub fn notify(item: &impl Summary) {
// Traits on function definitions.
// Only arguments that implement the Summary trait can use this function
    println!("Breaking news! {}", item.summarize());
}


// Trait Bound Syntax without syntax sugar:
pub fn notify_expanded<T: Summary>(item1: &T, item2: &T) {
    // This can be useful if you need to force multiple args to have the same trait
    println!("Breaking news! {}", item1.summarize());
    println!("Afterwards, {}", item2.summarize());
}


// Where Clauses
// If you have too many trait bounds, 
// a `where` clause can be used to declutter the function signature
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    42   
}


// You can also create a function that returns a value that must match a trait
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

