-- Add migration script here
CREATE TABLE violation (
    id INTEGER PRIMARY KEY NOT NULL,
    user_id INTEGER NOT NULL,
    violation VARCHAR(255) NOT NULL
);