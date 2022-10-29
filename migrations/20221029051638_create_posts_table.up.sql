CREATE TABLE IF NOT EXISTS `posts` (
    `id` VARCHAR(191) NOT NULL PRIMARY KEY,
    `title` VARCHAR(191) NOT NULL,
    `content` MEDIUMTEXT NOT NULL,
    `posted_at` DATETIME(3) NOT NULL,
    `updated_at` DATETIME(3)
);
