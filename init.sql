CREATE USER 'foyer'@'%' IDENTIFIED BY 'debug';
CREATE DATABASE foyer;
GRANT ALL PRIVILEGES ON foyer.* TO 'foyer'@'%';
FLUSH PRIVILEGES;

USE foyer;

CREATE TABLE IF NOT EXISTS users (
	login VARCHAR(64) PRIMARY KEY UNIQUE NOT NULL,
	img_url TEXT NOT NULL,
	grade INT NOT NULL
);

CREATE TABLE IF NOT EXISTS shifts (
    date_shift DATE NOT NULL,
    slot ENUM('day','night') NOT NULL,
    min_users INT DEFAULT 2,
    PRIMARY KEY (date_shift, slot)
);

CREATE TABLE IF NOT EXISTS shifts_user (
    date_shift DATE NOT NULL,
    slot ENUM('day','night') NOT NULL,
    login VARCHAR(64) NOT NULL REFERENCES users(login),
    PRIMARY KEY (date_shift, slot, login)
);
