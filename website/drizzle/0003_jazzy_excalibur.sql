CREATE TABLE `games` (
	`id` text PRIMARY KEY NOT NULL,
	`name` text NOT NULL,
	`image_path` text
);

--> statement-breakpoint
ALTER TABLE
	leagues
ADD
	`game_id` text REFERENCES games(id);