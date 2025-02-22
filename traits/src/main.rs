// The summarize method should be implemented on all members of Summary type
pub trait Summary {
    fn summarize(&self) -> String;
}
// Summarize can also be a default method implemented on all Summary types, that can then be overwritten later?
pub trait _Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

// We can also call methods on that same trait
pub trait __Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
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
fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    let pair = Pair::new(2, 2);
    pair.cmp_display();
}
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// another way for writing traits in function signature

pub fn _notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Implementing over mutliple params

//pub fn notify(item1: &impl Summary, item2: &impl Summary)
// pub fn notify<T: Summary>(item1: &T, item2: &T) {

// Implementing on diff params with diff types

//pub fn notify<T: Summary>(item1: &T, item2: &T) {

// Specify multiple traits on one param

//pub fn notify(item: &(impl Summary + Display)) {
//pub fn notify<T: Summary + Display>(item: &T) {

// Useing Where clause to make it easier to read

//fn some_function<T, U>(t: &T, u: &U) -> i32
//where
//T: Display + Clone,
//U: Clone + Debug,
//{

fn _returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// Cannot return multiple types tho

// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     }
//     else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
// cmp_display method will only be implemented on types that have both the Display and PartialOrd traits
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
