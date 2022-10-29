CREATE TABLE IF NOT EXISTS `tags` (
    `name` VARCHAR(191) NOT NULL,
    `post_id` VARCHAR(191) NOT NULL,
    PRIMARY KEY (`name`, `post_id`)
);
