ALTER TABLE `teams` RENAME COLUMN `image_path` TO `image_path_old`;
--> statement-breakpoint
ALTER TABLE `teams` ADD COLUMN `image_path` text;
--> statement-breakpoint
UPDATE `teams` SET `image_path` = `image_path_old`;
--> statement-breakpoint
ALTER TABLE `teams` DROP COLUMN `image_path_old`;
--> statement-breakpoint

ALTER TABLE `tracks` RENAME COLUMN `image_path` TO `image_path_old`;
--> statement-breakpoint
ALTER TABLE `tracks` ADD COLUMN `image_path` text;
--> statement-breakpoint
UPDATE `tracks` SET `image_path` = `image_path_old`;
--> statement-breakpoint
ALTER TABLE `tracks` DROP COLUMN `image_path_old`;