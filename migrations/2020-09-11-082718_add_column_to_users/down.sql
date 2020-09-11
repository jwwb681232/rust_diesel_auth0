-- This file should undo anything in `up.sql`
ALTER TABLE `users`
DROP COLUMN `password`,
DROP COLUMN `updated_at`
