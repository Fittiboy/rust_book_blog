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
    fn can_create_draft() {
        let post = Post::new();
        match post {
            DraftPost { .. } => assert!(true),
        }
    }

    #[test]
    fn can_add_text() {
        let mut post = Post::new();

        let to_add = String::from("I ate a salad for lunch today");

        post.add_text(&to_add);
        match post {
            DraftPost { content } if content == to_add => assert!(true),
            DraftPost { .. } => panic!(
                "Expected 'I ate a salad for lunch today' as content, got: {}",
                post.content
            ),
        };
    }

    #[test]
    fn can_request_review() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        let post = post.request_review();
        match post {
            WaitingPost { approvals: 0, .. } => assert!(true),
            WaitingPost { .. } => panic!("Should have no approvals, got: {}", post.approvals),
        };
    }

    #[test]
    fn can_approve_once() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        let post = post.request_review();
        let SomeApproval::StillWaiting(post) = post.approve() else {unreachable!(
            "Post will always be StillWaiting after a single approval"
        )};
        match post {
            WaitingPost { approvals: 1, .. } => assert!(true),
            WaitingPost { .. } => panic!("Should have one approval, got: {}", post.approvals),
        };
    }

    #[test]
    fn can_approve_twice() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        let post = post.request_review();
        let SomeApproval::StillWaiting(post) = post.approve() else {unreachable!(
            "Post will always be StillWaiting after a single approval"
        )};
        let SomeApproval::Approved(post) = post.approve() else {unreachable!(
            "Post will always be Approved after two approvals"
        )};
        match post {
            Post { .. } => assert!(true),
        };
    }

    #[test]
    fn can_get_content() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        let post = post.request_review();
        let SomeApproval::StillWaiting(post) = post.approve() else {unreachable!(
            "Post will always be StillWaiting after a single approval"
        )};
        let SomeApproval::Approved(post) = post.approve() else {unreachable!(
            "Post will always be Approved after two approvals"
        )};
        assert_eq!("I ate a salad for lunch today", post.content());
    }
}
