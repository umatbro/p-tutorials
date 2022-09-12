use std::future::Pending;

struct Post {
    content: String,
}
impl Post {
    fn new() -> DraftPost {
        DraftPost { content: String::new() }
    }

    fn content(&self) -> &String {
        &&self.content
    }

}

struct DraftPost {
    content: String,
}

impl DraftPost {
    fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    fn request_review(self) -> PendingReviewPost {
        PendingReviewPost { content: self.content }
    }
}

struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    fn approve(self) -> Post {
        Post {
            content: self.content
        }
    }
}

pub fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    let post = post.request_review();
    let post = post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
