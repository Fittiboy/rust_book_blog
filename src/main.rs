use blog::{
    Post,
    SomeApproval::{Approved, StillWaiting},
};

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    let post = post.request_review();
    let StillWaiting(post) = post.approve() else {unreachable!(
        "Post will always be StillWaiting after a single approval"
    )};
    let Approved(post) = post.approve() else {unreachable!(
        "Post will always be Approved after two approvals"
    )};
    assert_eq!("I ate a salad for lunch today", post.content());
}
