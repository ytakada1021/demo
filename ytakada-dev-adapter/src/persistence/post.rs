use anyhow::{Result, Ok};
use async_trait::async_trait;
use sqlx::{MySqlPool, query};
use ytakada_dev_core::post::Post;

#[async_trait]
pub trait PostRepository {
    async fn save(&self, post: &Post) -> Result<()>;
}

pub struct MysqlPostRepository {
    pub pool: MySqlPool,
}

#[async_trait]
impl PostRepository for MysqlPostRepository {
    async fn save(&self, post: &Post) -> Result<()> {
        let id = post.id().value();
        let title = post.title().value();
        let content = post.content().value();
        let posted_at = post.posted_at();
        let updated_at = post.updated_at();

        query("
            INSERT INTO `posts`
                (`id`, `title`, `content`, `posted_at`, `updated_at`)
            VALUES
                (?, ?, ?, ?, ?)
            ON DUPLICATE KEY UPDATE
                `title` = ?,
                `content` = ?,
                `posted_at` = ?,
                `updated_at` = ?
        ")
        // VALUES
        .bind(id)
        .bind(title)
        .bind(content)
        .bind(posted_at)
        .bind(updated_at)
        // ON DUPLICATE KEY UPDATE
        .bind(title)
        .bind(content)
        .bind(posted_at)
        .bind(updated_at)
        //
        .execute(&self.pool)
        .await
        .unwrap();

        query("
            DELETE FROM `tags`
            WHERE
                `tags`.`post_id` = ?
        ")
        .bind(id)
        .execute(&self.pool)
        .await
        .unwrap();

        for tag in post.tags() {
            query("
                INSERT INTO `tags`
                    (`name`, `post_id`)
                VALUES
                    (?, ?)
            ")
            .bind(tag.value())
            .bind(id)
            .execute(&self.pool)
            .await
            .unwrap();
        }

        Ok(())
    }
}

#[tokio::test]
#[ignore]
async fn test_connection() {
    let pool = MySqlPool::connect("mysql://root:password@localhost:3306/local").await.unwrap();

    query!(r#"
        INSERT INTO `posts`(`id`, `title`, `content`, `posted_at`, `updated_at`)
        VALUES ('sample', 'Changed Title', '<h1>Sample Content</h1>', '2022-10-01 18:00:00', NULL)
        ON DUPLICATE KEY UPDATE `title` = 'Changed Title'
    "#)
    .execute(&pool)
    .await
    .unwrap();
}
