-- This file should undo anything in `up.sql`
ALTER TABLE `dashboards` DROP COLUMN `subtitle`;
ALTER TABLE `dashboards` DROP COLUMN `project_id`;

DROP TABLE IF EXISTS `variables`;
DROP TABLE IF EXISTS `projects`;
DROP TABLE IF EXISTS `components`;
