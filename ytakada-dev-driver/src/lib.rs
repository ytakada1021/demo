use ytakada_dev_core::post::{Post, PostId};

pub trait PostDao {
    fn find(id: PostId) -> Option<Post>;
    fn save(post: Post);
    fn delete(post: Post);
}

pub struct MongoPostDao;
