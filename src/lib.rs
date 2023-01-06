use std::error::Error;

mod states;

pub struct Post {
    state: Box<dyn states::State>,
    text: String,
}

impl Post {
    pub fn new() -> Self {
        Post {
            state: Box::new(states::Draft {}),
            text: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) -> Result<(), Box<dyn Error>> {
        self.state.add_text()?;
        self.text.push_str(text);
        Ok(())
    }

    pub fn request_review(&mut self) -> Result<(), Box<dyn Error>> {
        self.state.request_review()?;
        self.state = Box::new(states::AwaitingReview {});
        Ok(())
    }

    pub fn approve(&mut self) -> Result<(), Box<dyn Error>> {
        self.state.approve()?;
        self.state = Box::new(states::Posted {});
        Ok(())
    }

    pub fn reject(&mut self) -> Result<(), Box<dyn Error>> {
        self.state.reject()?;
        self.state = Box::new(states::Draft {});
        Ok(())
    }

    pub fn content(&self) -> Result<&str, Box<dyn Error>> {
        self.state.content()?;
        Ok(&self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn cannot_read_draft() {
        let post = Post::new();
        post.content().unwrap();
    }

    #[test]
    #[should_panic]
    fn cannot_reject_draft() {
        let mut post = Post::new();
        post.reject().unwrap();
    }

    #[test]
    fn can_approve_and_reject() {
        let mut post = Post::new();
        post.request_review().unwrap();
        post.reject().unwrap();
        post.request_review().unwrap();
        post.approve().unwrap();
    }

    #[test]
    fn can_post_and_read() {
        let mut post = Post::new();
        post.add_text("Hello, world!").unwrap();
        post.request_review().unwrap();
        post.approve().unwrap();
        assert_eq!("Hello, world!", post.content().unwrap());
    }
}
