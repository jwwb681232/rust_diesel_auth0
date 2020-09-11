-- Your SQL goes here
ALTER TABLE `users` ADD COLUMN `password` varchar(255) NOT NULL AFTER `email`;
ALTER TABLE `users` ADD COLUMN `updated_at` timestamp NOT NULL ON UPDATE CURRENT_TIMESTAMP AFTER `created_at`;
