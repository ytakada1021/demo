use anyhow::{Result, Ok};
use ytakada_dev_core::post::{Post, PostId};
use ytakada_dev_driver::{find, save};

pub async fn save_post(post_id: impl Into<String>, markdown: &impl Into<String>) -> Result<Post> {
    let post_id = PostId::new(post_id).unwrap();

    let post = match find(&post_id).await.unwrap() { // Read post from persistence.
        Some(post) => {
            Post::from_markdown(post_id, markdown) // Update if post already exists.
        },
        None => {
            Post::from_markdown(post_id, markdown) // Create if post does not exist.
        }
    };

    // Save to persistence.
    save(&post).await.unwrap();

    Ok(post)
}
