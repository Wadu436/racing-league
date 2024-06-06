CREATE TABLE `drivers` (
	`id` text PRIMARY KEY NOT NULL,
	`name` text NOT NULL,
	`bot` integer NOT NULL,
	`country` text(2) NOT NULL
);
--> statement-breakpoint
CREATE TABLE `event_sessions` (
	`id` text PRIMARY KEY NOT NULL,
	`session_type` text NOT NULL,
	`event_id` text NOT NULL,
	`fastest_lap` integer,
	FOREIGN KEY (`event_id`) REFERENCES `events`(`id`) ON UPDATE no action ON DELETE cascade,
	FOREIGN KEY (`fastest_lap`) REFERENCES `drivers`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE TABLE `events` (
	`id` text PRIMARY KEY NOT NULL,
	`name` text NOT NULL,
	`league_order` integer NOT NULL,
	`date` integer,
	`league_id` text NOT NULL,
	`track_id` text NOT NULL,
	FOREIGN KEY (`league_id`) REFERENCES `leagues`(`id`) ON UPDATE no action ON DELETE cascade,
	FOREIGN KEY (`track_id`) REFERENCES `tracks`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE TABLE `games` (
	`id` text PRIMARY KEY NOT NULL,
	`name` text NOT NULL,
	`image_path` text
);
--> statement-breakpoint
CREATE TABLE `laps` (
	`session_id` text NOT NULL,
	`driver_id` text NOT NULL,
	`lap_number` integer NOT NULL,
	`laptime_in_ms` integer NOT NULL,
	`sector_1_time_in_ms` integer NOT NULL,
	`sector_2_time_in_ms` integer NOT NULL,
	`sector_3_time_in_ms` integer NOT NULL,
	`valid` integer NOT NULL,
	`in_lap` integer NOT NULL,
	`out_lap` integer NOT NULL,
	`safety_car` integer NOT NULL,
	`virtual_safety_car` integer NOT NULL,
	PRIMARY KEY(`driver_id`, `lap_number`, `session_id`),
	FOREIGN KEY (`session_id`) REFERENCES `sessions`(`id`) ON UPDATE no action ON DELETE no action,
	FOREIGN KEY (`driver_id`) REFERENCES `drivers`(`id`) ON UPDATE no action ON DELETE no action,
	FOREIGN KEY (`session_id`,`driver_id`) REFERENCES `session_entries`(`session_id`,`driver_id`) ON UPDATE no action ON DELETE cascade
);
--> statement-breakpoint
CREATE TABLE `league_entries` (
	`league_id` text NOT NULL,
	`driver_id` text NOT NULL,
	`team_id` text NOT NULL,
	PRIMARY KEY(`driver_id`, `league_id`),
	FOREIGN KEY (`league_id`) REFERENCES `leagues`(`id`) ON UPDATE no action ON DELETE cascade,
	FOREIGN KEY (`driver_id`) REFERENCES `drivers`(`id`) ON UPDATE no action ON DELETE no action,
	FOREIGN KEY (`team_id`) REFERENCES `teams`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE TABLE `leagues` (
	`id` text PRIMARY KEY NOT NULL,
	`name` text NOT NULL,
	`status` text NOT NULL,
	`game_id` text,
	FOREIGN KEY (`game_id`) REFERENCES `games`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE TABLE `new_users` (
	`id` text PRIMARY KEY NOT NULL,
	`provider_id` text NOT NULL,
	`provider_user_id` text NOT NULL,
	`expiration_time` integer NOT NULL
);
--> statement-breakpoint
CREATE TABLE `oauth_accounts` (
	`provider_id` text NOT NULL,
	`provider_user_id` text NOT NULL,
	`user_id` text NOT NULL,
	PRIMARY KEY(`provider_id`, `provider_user_id`),
	FOREIGN KEY (`user_id`) REFERENCES `users`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE TABLE `overtakes` (
	`id` text PRIMARY KEY NOT NULL,
	`session_id` text NOT NULL,
	`overtaking_driver_id` text NOT NULL,
	`overtaken_driver_id` text NOT NULL,
	`lap_number` integer NOT NULL,
	FOREIGN KEY (`session_id`) REFERENCES `sessions`(`id`) ON UPDATE no action ON DELETE cascade,
	FOREIGN KEY (`overtaking_driver_id`) REFERENCES `drivers`(`id`) ON UPDATE no action ON DELETE no action,
	FOREIGN KEY (`overtaken_driver_id`) REFERENCES `drivers`(`id`) ON UPDATE no action ON DELETE no action,
	FOREIGN KEY (`session_id`,`overtaking_driver_id`) REFERENCES `session_entries`(`session_id`,`driver_id`) ON UPDATE no action ON DELETE no action,
	FOREIGN KEY (`session_id`,`overtaken_driver_id`) REFERENCES `session_entries`(`session_id`,`driver_id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE TABLE `session_entries` (
	`session_id` text NOT NULL,
	`driver_id` text NOT NULL,
	`team_id` text NOT NULL,
	`finish_status` text NOT NULL,
	`grid_position` integer NOT NULL,
	`finish_position` integer NOT NULL,
	`grid_penalty` integer DEFAULT 0 NOT NULL,
	`fastest_lap` integer NOT NULL,
	`total_time_without_penalties_in_ms` integer NOT NULL,
	`penalty_time_in_s` integer NOT NULL,
	FOREIGN KEY (`session_id`) REFERENCES `sessions`(`id`) ON UPDATE no action ON DELETE cascade,
	FOREIGN KEY (`driver_id`) REFERENCES `drivers`(`id`) ON UPDATE no action ON DELETE no action,
	FOREIGN KEY (`team_id`) REFERENCES `teams`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE TABLE `sessions` (
	`id` text PRIMARY KEY NOT NULL,
	`user_id` text NOT NULL,
	`expires_at` integer NOT NULL,
	FOREIGN KEY (`user_id`) REFERENCES `users`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE TABLE `teams` (
	`id` text PRIMARY KEY NOT NULL,
	`name` text NOT NULL,
	`country` text(2) NOT NULL,
	`image_path` text
);
--> statement-breakpoint
CREATE TABLE `tracks` (
	`id` text PRIMARY KEY NOT NULL,
	`name` text NOT NULL,
	`country` text(2) NOT NULL,
	`image_path` text
);
--> statement-breakpoint
CREATE TABLE `tyre_stints` (
	`session_id` text NOT NULL,
	`driver_id` text NOT NULL,
	`order` integer NOT NULL,
	`compound` text NOT NULL,
	`start_lap` integer NOT NULL,
	`end_lap` integer NOT NULL,
	PRIMARY KEY(`driver_id`, `order`, `session_id`),
	FOREIGN KEY (`session_id`) REFERENCES `sessions`(`id`) ON UPDATE no action ON DELETE no action,
	FOREIGN KEY (`driver_id`) REFERENCES `drivers`(`id`) ON UPDATE no action ON DELETE no action,
	FOREIGN KEY (`session_id`,`driver_id`) REFERENCES `session_entries`(`session_id`,`driver_id`) ON UPDATE no action ON DELETE cascade
);
--> statement-breakpoint
CREATE TABLE `users` (
	`id` text PRIMARY KEY NOT NULL,
	`username` text NOT NULL,
	`admin` integer NOT NULL,
	`staff` integer NOT NULL
);
--> statement-breakpoint
CREATE UNIQUE INDEX `drivers_name_unique` ON `drivers` (`name`);--> statement-breakpoint
CREATE UNIQUE INDEX `users_username_unique` ON `users` (`username`);