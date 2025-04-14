-- Create transactions table to record all financial transactions
-- Each transaction is linked to a user and a specific category

CREATE TABLE `transactions` (
    `id`          BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `user_id`     BIGINT UNSIGNED NOT NULL,
    `category_id` BIGINT UNSIGNED NOT NULL,
    `type`        VARCHAR(255)    NOT NULL,
    `amount`      BIGINT UNSIGNED NOT NULL,
    `description` TEXT,
    `balance`     BIGINT UNSIGNED NOT NULL DEFAULT 0,
    `created_at`  DATETIME        NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at`  DATETIME        NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    
    PRIMARY KEY (`id`),
    FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE,
    FOREIGN KEY (`category_id`) REFERENCES `categories` (`id`) ON DELETE CASCADE,
    INDEX `idx_user_transactions` (`user_id`),
    INDEX `idx_category_transactions` (`category_id`)
);