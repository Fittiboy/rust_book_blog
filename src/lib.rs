pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

pub struct WaitingPost {
    content: String,
    approvals: u8,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> WaitingPost {
        WaitingPost {
            content: self.content,
            approvals: 0,
        }
    }
}

pub enum SomeApproval {
    StillWaiting(WaitingPost),
    Approved(Post),
}

impl WaitingPost {
    pub fn approve(mut self) -> SomeApproval {
        match self.approvals {
            0 => {
                self.approvals = 1;
                SomeApproval::StillWaiting(self)
            }
            _ => SomeApproval::Approved(Post {
                content: self.content,
            }),
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_approve_and_reject() {
        let mut post = Post::new();
        post.add_text("Hello");
        let post = post.request_review();
        let post = post.reject();
        let post = post.request_review();
        let post = post.approve();
        assert_eq!("Hello", post.content());
    }

    #[test]
    fn can_post_and_read() {
        let mut post = Post::new();
        post.add_text("Hello, world!");
        let post = post.request_review();
        let post = post.approve();
        assert_eq!("Hello, world!", post.content());
    }
}
