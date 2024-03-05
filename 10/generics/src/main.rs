#![allow(unused)]

use std::fmt::{Display, Formatter, Result};

pub trait Summary {
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
    fn summarize_author(&self) -> String {
        format!("(Read more from {}...)", self.author)
    }
}

impl Display for NewsArticle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "({}, {}, {}, {})",
            self.headline, self.location, self.author, self.content
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
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

impl Display for Tweet {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "({}, {}, {}, {})",
            self.username, self.content, self.reply, self.retweet
        )
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
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

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    notify(&article);
}

pub fn notify<S: Summary + Display>(item: &(S)) {
    println!("Breaking news! {}", item.summarize());
}

// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

// struct Point2<T, U> {
//     x: T,
//     y: U,
// }

// fn main() {
//     // Generic functions
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("The largest number is {}", result);

//     let char_list = vec!['y', 'm', 'a', 'q'];
//     let result = largest(&char_list);
//     println!("The largest char is {}", result);

//     // Generic structs
//     let interger = Point { x: 5, y: 10 };
//     let float = Point { x: 1.0, y: 4.0 };
//     let integer_and_float = Point2 { x: 5, y: 4.0 };

//     println!("x of integer: {}", interger.x());
// }

// fn largest<T>(list: &[T]) -> T
// where
//     T: PartialOrd + Copy,
// {
//     let mut largest = list[0];

//     for &number in list {
//         if number > largest {
//             largest = number;
//         }
//     }

//     largest
// }

// fn largest(list: &[i32]) -> i32 {
//     let mut largest = list[0];

//     for &number in list {
//         if number > largest {
//             largest = number;
//         }
//     }

//     largest
// }
