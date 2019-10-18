use std::fmt::{Debug, Error, Formatter};


trait State: Debug {
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    fn approve(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

#[derive(Debug)]
struct Draft {}

#[derive(Debug)]
struct PendingReview {}

#[derive(Debug)]
struct Published {}

impl Draft {
    fn new() -> Self {
        Self {}
    }
}

impl PendingReview {
    fn new() -> Self {
        Self {}
    }
}

impl Published {
    fn new() -> Self {
        Self{}
    }
}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview::new())
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
        Box::new(Published::new())
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

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Debug for Post {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        write!(fmt, "Post {{ content: {:?}, state: {:?} }}", self.content, self.state.as_ref().unwrap())?;
        Ok(())
    }
}

impl Post {
    fn new() -> Self {
        Self {
            state: Some(Box::new(Draft::new())),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn it_works() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());

        println!("{:?} = ", post);
    }
}