extern crate blog;
use blog::Post;

fn main() {
    let mut post = Post::new();

    assert_eq!("", post.content());
    assert_eq!("draft", post.state());

    post.add_text("I ate a salad for lunch today").expect(
        "Error adding text",
    );

    assert_eq!("", post.content());
    assert_eq!("draft", post.state());

    post.request_review();

    assert_eq!("", post.content());
    assert_eq!("pending", post.state());

    post.reject();

    assert_eq!("", post.content());
    assert_eq!("draft", post.state());

    post.request_review();

    assert_eq!("", post.content());
    assert_eq!("pending", post.state());

    post.approve();

    assert_eq!("", post.content());
    assert_eq!("pending", post.state());

    post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
    assert_eq!("published", post.state());
}