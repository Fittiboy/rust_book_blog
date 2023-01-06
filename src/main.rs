use blog::{
    Post,
    SomeApproval::{Approved, StillWaiting},
};

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    let post = post.request_review();
    let post = post.approve();
    if let StillWaiting(waiting) = post {
        let post = waiting.approve();
        if let Approved(approved) = post {
            let post = approved;
            assert_eq!("I ate a salad for lunch today", post.content());
        }
    }
}
