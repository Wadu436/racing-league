ALTER TABLE `events` RENAME COLUMN `date` TO `date_old`;
--> statement-breakpoint
ALTER TABLE `events` ADD COLUMN `date` integer;
--> statement-breakpoint
UPDATE `events` SET `date` = `date_old`;
--> statement-breakpoint
ALTER TABLE `events` DROP COLUMN `date_old`;