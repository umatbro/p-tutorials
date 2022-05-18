mod aggregator;
use aggregator::{Summary, Tweet, NewsArticle};


fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let point = Point{x: 1, y: 2};
    println!("{:?}", point);
    println!("{}", point.x());
    traits();
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn traits() {
    let tweet = Tweet {
        content: String::from("No can do"),
        username: String::from("umat"),
        retweet: true,
        reply: false,
    };
    let news_art = NewsArticle {
        author: String::from("Mat"),
        content: String::from("Test content"),
        headline: String::from("Headline"),
        location: String::from("Unknown"),
    };
    println!("{}", tweet.summarize());
    println!("{}", tweet);
    println!("{}", tweet.to_string());
}