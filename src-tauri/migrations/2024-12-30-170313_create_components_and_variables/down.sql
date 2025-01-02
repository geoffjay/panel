-- This file should undo anything in `up.sql`
ALTER TABLE `dashboards` DROP COLUMN `subtitle`;

DROP TABLE IF EXISTS `components`;
DROP TABLE IF EXISTS `variables`;
