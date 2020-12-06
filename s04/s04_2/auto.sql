-- CREATE TABLE `user` (
--     `id` INT UNSIGNED AUTO_INCREMENT,
--     `account` VARCHAR(255) NOT NULL COMMNET "acount",
--     `password` CHAR(32) NOT NULL COMMENT "password",
--     PRIMARY KEY(`id`),
--     UNIQUE KEY `account`(`account`)
-- )ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=UTF8;

CREATE TABLE `users` (
 `id` BIGINT NOT NULL AUTO_INCREMENT,
 `account` VARCHAR(255) NOT NULL COMMENT "account",
 `password` CHAR(32) NOT NULL COMMENT "password",
 PRIMARY KEY(`id`),
 UNIQUE KEY`account`(`account`)
)ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=UTF8;