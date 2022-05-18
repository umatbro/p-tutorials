use std::fmt::{Debug, Display, Error, Formatter, Result};

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub retweet: bool,
    pub reply: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
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

impl Display for Tweet {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.content.is_empty() {
            return Err(Error);
        }
        write!(f, "Tweet(un={}, con={})", self.username, self.content)
    }
}

impl ToString for NewsArticle {
    fn to_string(&self) -> String {
        format!("NewsArticle(h={})", self.headline)
    }
}
