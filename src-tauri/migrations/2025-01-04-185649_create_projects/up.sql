-- Your SQL goes here
ALTER TABLE `dashboards` ADD COLUMN `subtitle` TEXT NOT NULL;
ALTER TABLE `dashboards` ADD COLUMN `project_id` INTEGER NOT NULL;

CREATE TABLE `variables`(
	`id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	`ref_id` VARCHAR,
	`default` TEXT,
	`value` TEXT NOT NULL,
	`dashboard_id` INTEGER NOT NULL,
	`project_id` INTEGER NOT NULL,
	FOREIGN KEY (`dashboard_id`) REFERENCES `dashboards`(`id`),
	FOREIGN KEY (`project_id`) REFERENCES `projects`(`id`)
);

CREATE TABLE `projects`(
	`id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	`title` TEXT NOT NULL,
	`subtitle` TEXT NOT NULL,
	`description` TEXT NOT NULL
);

CREATE TABLE `components`(
	`id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	`dashboard_id` INTEGER,
	`parent_id` INTEGER,
	`type` TEXT NOT NULL,
	FOREIGN KEY (`dashboard_id`) REFERENCES `dashboards`(`id`)
);

