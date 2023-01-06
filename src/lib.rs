use std::error::Error;

#[derive(Debug)]
struct Draft {}
#[derive(Debug)]
struct AwaitingReview {}
#[derive(Debug)]
struct Posted {}

trait State
where
    Self: std::fmt::Debug,
{
    fn add_text(&self) -> Result<(), Box<dyn Error>> {
        Err(format!("Expected Post in Draft, found state {:?}", self).into())
    }

    fn request_review(&self) -> Result<(), Box<dyn Error>> {
        Err(format!("Expected Post in Draft, found state {:?}", self).into())
    }

    fn approve(&self) -> Result<(), Box<dyn Error>> {
        Err(format!("Expected Post in AwaitingReview, found state {:?}", self).into())
    }

    fn content(&self) -> Result<(), Box<dyn Error>> {
        Err(format!("Expected Post in Posted, found state {:?}", self).into())
    }
}

impl State for Draft {
    fn add_text(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn request_review(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

impl State for AwaitingReview {
    fn approve(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

impl State for Posted {
    fn content(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

pub struct Post {
    state: Box<dyn State>,
    text: String,
}

impl Post {
    pub fn new() -> Self {
        Post {
            state: Box::new(Draft {}),
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
        self.state = Box::new(AwaitingReview {});
        Ok(())
    }

    pub fn approve(&mut self) -> Result<(), Box<dyn Error>> {
        self.state.approve()?;
        self.state = Box::new(Posted {});
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
    fn cannot_approve_draft() {
        let mut post = Post::new();
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
