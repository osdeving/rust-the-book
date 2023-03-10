#![allow(unused, dead_code)]

use std::error::Error;

use rust_the_book::{Tweet, Summary, NewsArticle};



fn main() -> Result<(), Box<dyn Error>> {

    let tweet = Tweet {
        username: "Willams".to_string(),
        content: "Test of traits".to_string(),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());


    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    Ok(())
}