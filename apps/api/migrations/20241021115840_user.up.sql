-- Add up migration script here

DROP TABLE IF EXISTS USERS;

CREATE TABLE USERS (
    username TEXT NOT NULL,
    email TEXT NOT NULL,
    password TEXT NOT NULL,
    name TEXT NOT NULL
);