use blog::Post;

fn main() {
    let mut post = Post::new();
    
    post.add_text("Today dinosaurs escaped from planet Earth");

    println!("Post:\n{}\n", post.content());
    post.request_review();
    post.approve();
    println!("Post:\n{}\n", post.content());
    post.approve();
    println!("Post:\n{}\n", post.content());
}
