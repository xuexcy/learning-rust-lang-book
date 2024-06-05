use blog::{Post, PostV2};

fn main() {
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
    post.request_review();
    assert_eq!("", post.content());
    post.approve();
    assert_eq!("", post.content());
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    let mut post = PostV2::new();
    post.add_text("I ate a salad");
    let post = post.request_review();
    let post = post.approve();
    assert_eq!("I ate a salad", post.content());
}
