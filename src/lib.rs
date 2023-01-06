use std::error::Error;

#[derive(Debug)]
enum PostState {
    Draft,
    AwaitingReview,
    Posted,
}

pub struct Post {
    state: PostState,
    text: String,
}

impl Post {
    pub fn new() -> Self {
        Post {
            state: PostState::Draft,
            text: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) -> Result<(), Box<dyn Error>> {
        match self.state {
            PostState::Draft => {
                self.text.push_str(text);
                Ok(())
            }
            _ => Err(format!("Expected Post in Draft, got state: {:?}", self.state).into()),
        }
    }

    pub fn request_review(&mut self) -> Result<(), Box<dyn Error>> {
        match self.state {
            PostState::Draft => {
                self.state = PostState::AwaitingReview;
                Ok(())
            }
            _ => Err(format!("Expected Post in Draft, got state: {:?}", self.state).into()),
        }
    }

    pub fn approve(&mut self) -> Result<(), Box<dyn Error>> {
        match self.state {
            PostState::AwaitingReview => {
                self.state = PostState::Posted;
                Ok(())
            }
            _ => Err(format!(
                "Expected Post in AwaitingReview, got state: {:?}",
                self.state
            )
            .into()),
        }
    }

    pub fn content(&self) -> Result<&str, Box<dyn Error>> {
        match self.state {
            PostState::Posted => Ok(&self.text),
            _ => Err(format!("Expected Post in Posted, got state: {:?}", self.state).into()),
        }
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
