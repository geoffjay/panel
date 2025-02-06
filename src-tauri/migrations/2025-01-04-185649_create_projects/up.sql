-- Your SQL goes here
ALTER TABLE `dashboards` ADD COLUMN `project_id` INTEGER NOT NULL;

CREATE TABLE `projects`(
	`id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	`name` TEXT NOT NULL,
	`description` TEXT NOT NULL,
	`path` TEXT NOT NULL
);
