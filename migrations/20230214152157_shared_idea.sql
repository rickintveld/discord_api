-- Add migration script here
CREATE TABLE shared_idea (
    id INTEGER PRIMARY KEY NOT NULL,
    user_id INTEGER NOT NULL,
    url VARCHAR(255) NOT NULL
);