/*
You're trying to delete PRIMARY KEY(driver_id,session_id) from 'session_entries' table
SQLite does not supportprimary key deletion from existing table
You can do it in 3 steps with drizzle orm:
 - create new mirror table table without pk, rename current table to old_table, generate SQL
 - migrate old data from one table to another
 - delete old_table in schema, generate sql

or create manual migration like below:

ALTER TABLE table_name RENAME TO old_table;
CREATE TABLE table_name (
	column1 datatype [ NULL | NOT NULL ],
	column2 datatype [ NULL | NOT NULL ],
	...
	PRIMARY KEY (pk_col1, pk_col2, ... pk_col_n)
 );
INSERT INTO table_name SELECT * FROM old_table;

Due to that we don't generate migration automatically and it has to be done manually
*/
--> statement-breakpoint
ALTER TABLE `session_entries` ADD `grid_penalty` integer DEFAULT 0 NOT NULL;--> statement-breakpoint
ALTER TABLE `session_entries` ADD `fastest_lap` integer NOT NULL;--> statement-breakpoint
ALTER TABLE `event_sessions` DROP COLUMN `event_order`;