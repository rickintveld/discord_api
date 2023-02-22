-- Add migration script here
CREATE TABLE profit (
    id INTEGER PRIMARY KEY NOT NULL,
    user_id BIGINT NOT NULL,
    profit REAL NOT NULL,
    risk_to_reward REAL NOT NULL,
    creation_date VARCHAR(10) NOT NULL
);