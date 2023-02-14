-- Add migration script here
CREATE TABLE competition_winner (
    id INTEGER PRIMARY KEY NOT NULL,
    user_id INTEGER NOT NULL,
    number INTEGER NOT NULL
);