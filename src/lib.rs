#![allow(unused, dead_code)]

use std::{error::Error, fmt::Display, fmt::Debug};

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        "(Read more...)".to_string()
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub author: String,
    pub content: String,
    pub location: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

//impl Summary for NewsArticle {}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// long form
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }


pub fn notify2(item: &(impl Summary + Display)) {
}


fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    0
}

fn some_function2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug, {
    0
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("The largest number is x = {}", self.x);
        } else {
            println!("The largest number is y = {}", self.y);
        }
    }
}
