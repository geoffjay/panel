-- Your SQL goes here
ALTER TABLE `dashboards` ADD COLUMN `project_id` INTEGER NOT NULL;

CREATE TABLE `projects`(
	`id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	`title` TEXT NOT NULL,
	`subtitle` TEXT NOT NULL,
	`description` TEXT NOT NULL
);
