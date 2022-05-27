use std::fmt::Debug;

pub fn lifetimes() {
    let first = String::from("asdf");
    let second = "xyz";
    println!("Longest string is: {}", longest(first.as_str(), second))
}

fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() > second.len() {
        first
    } else {
        second
    }
}

// Lifetime annotationsn in Struct Definitions
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention, please: {}", announcement);
        self.part
    }
}

fn novel() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split(".").next().expect("Could not find a sentence");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

fn first_word(x: &str) -> &str {
    "abc"
}

fn static_lifetime() {
    let s: &'static str = "I have a static lifetime.";
}

fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str 
where 
    T: Debug
{
    println!("Announcement! {:?}", ann);
    
    if x.len() > y.len() {
        x
    } else {
        y
    }
}