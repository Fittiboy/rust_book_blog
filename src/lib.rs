#[derive(Debug, PartialEq)]
pub struct NoApproval;
#[derive(Debug, PartialEq)]
pub struct Approval;

pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

pub struct WaitingPost<T> {
    content: String,
    _approval: T,
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

    pub fn request_review(self) -> WaitingPost<NoApproval> {
        WaitingPost {
            content: self.content,
            _approval: NoApproval {},
        }
    }
}

impl WaitingPost<NoApproval> {
    pub fn approve(self) -> WaitingPost<Approval> {
        WaitingPost {
            content: self.content,
            _approval: Approval,
        }
    }
}

impl WaitingPost<Approval> {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

impl<T> WaitingPost<T> {
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
    fn can_create_draft() {
        let post = Post::new();
        assert_eq!(post.content, "")
    }

    #[test]
    fn can_add_text() {
        let mut post = Post::new();

        let to_add = String::from("I ate a salad for lunch today");

        post.add_text(&to_add);
        assert_eq!(post.content, to_add);
    }

    #[test]
    fn can_request_review() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        let post = post.request_review();
        assert_eq!(post._approval, NoApproval {});
    }

    #[test]
    fn can_approve_once() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        let post = post.request_review().approve();
        assert_eq!(post._approval, Approval {});
    }

    #[test]
    fn can_approve_twice() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        let post = post.request_review().approve().approve();
        // The content method can only be called on the type we want to find
        assert_ne!("", post.content());
    }

    #[test]
    fn can_get_content() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        let post = post.request_review().approve().approve();
        assert_eq!("I ate a salad for lunch today", post.content());
    }
}
