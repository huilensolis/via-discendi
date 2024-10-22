-- Add up migration script here

DROP TABLE IF EXISTS USERS_SESSION;

CREATE TABLE USERS_SESSION (
    token TEXT NOT NULL PRIMARY KEY,
    refresh_token TEXT NOT NULL UNIQUE,
    expiry_date DATE NOT NULL,
    username TEXT NOT NULL UNIQUE
);