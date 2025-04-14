-- Create users table to store user account information
-- This table handles basic user authentication and profile data

CREATE TABLE `users` (
    `id`         BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `first_name` VARCHAR(255)    NOT NULL,
    `last_name`  VARCHAR(255)    NOT NULL,
    `balance`    BIGINT UNSIGNED NOT NULL     0,
    `email`      VARCHAR(255)    NOT NULL UNIQUE,
    `password`   VARCHAR(255)    NOT NULL,
    `created_at` DATETIME        NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at` DATETIME        NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    
    PRIMARY KEY (`id`),
    INDEX `idx_email` (`email`)
);