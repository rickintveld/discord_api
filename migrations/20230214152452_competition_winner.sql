-- Add migration script here
CREATE TABLE competition_winner (
    id INTEGER PRIMARY KEY NOT NULL,
    user_id INTEGER NOT NULL,
    rank INTEGER NOT NULL,
    price INTEGER NOT NULL,
    prop_firm VARCHAR(50) NOT NULL
);