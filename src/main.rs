use blog::Post;

fn main() {
    let mut post = Post::new();
    
    post.add_text("Today dinosaurs escaped from planet Earth.");

    // At this point, the post should not print because it's a draft
    println!("Post:\n{}\n", post.content());

    post.request_review();
    post.approve();

    // Since posts require double approval, the content still won't print yet
    println!("Post:\n{}\n", post.content());
    post.approve();

    // Now that the post has been approved twice, it is published and the content will be printed
    println!("Post:\n{}\n", post.content());

    // The post is not in draft mode, so adding text should have no effect here
    post.add_text("First attempt to add new text");

    // The post should now be in Pending Review
    post.reject();

    // The post isn't in draft yet, so this should have no effect
    post.add_text("Second attempt to add new text");

    // The post should now be back to Draft
    post.reject();

    // The post should accept the new text now
    post.add_text("\nCaptain T-Rex was quoted as saying \"Third time is a charm!\"");

    // Two editors approve of this post after review
    post.request_review();
    post.approve();
    post.approve();

    // Let's see the updated post
    println!("Post:\n{}\n", post.content());
}
