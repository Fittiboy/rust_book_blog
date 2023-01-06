use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today")
        .unwrap_or_else(|err| {
            println!("{}", err);
        });
    assert_eq!("", post.content().unwrap_or(""));

    post.request_review().unwrap_or_else(|err| {
        println!("{}", err);
    });
    assert_eq!("", post.content().unwrap_or(""));

    post.approve().unwrap_or_else(|err| {
        println!("{}", err);
    });
    assert_eq!(
        "I ate a salad for lunch today",
        post.content().unwrap_or("")
    );
}
