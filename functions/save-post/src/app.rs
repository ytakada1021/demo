use ytakada_dev_core::post::{Post, PostId};

pub fn save_post(post_id: impl Into<String>, markdown: &impl Into<String>) -> Post {
    // Read domain model from persistence

    // Update domain model
    Post::from_markdown(PostId::new(post_id).unwrap(), markdown)

    // Save domain model to persistence
}
