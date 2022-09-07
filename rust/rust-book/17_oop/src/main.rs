use core::fmt::Debug;

fn main() {
    let mut post = Post::new();
    post.add_text(String::from("Hello, article!"));
    println!("Content at start: {}", post.content());
    post.request_review();
    println!("Content at request review: {}", post.content());

    post.approve();
    println!("Content at approve: {}", post.content());
    post.request_review();
    println!("Content after attempt to move back to request_review: {}", post.content());
}

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    fn new() -> Post {
        Post {
            state: Some(Box::new(Draft)),
            content: String::new(),
        }
    }

    fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn add_text(&mut self, text: String) {
        self.content.push_str(&text)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }


    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}


trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

#[derive(Debug)]
struct Draft;
struct PendingReview;
struct Published;

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published{})
    }
}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}