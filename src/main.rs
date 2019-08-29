use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("Dinosaurs escaped from Earth");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("Dinosaurs escaped from Earth", post.content());
    println!("{}", post.content());
}
