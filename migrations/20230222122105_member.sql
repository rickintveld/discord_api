-- Add migration script here
CREATE TABLE member (
    id INTEGER PRIMARY KEY NOT NULL,
    user_id INTEGER NOT NULL,
    username VARCHAR(50) NOT NULL,
    creation_date VARCHAR(10) NOT NULL,
    UNIQUE (user_id) 
);