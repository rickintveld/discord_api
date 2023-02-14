-- Add migration script here
CREATE TABLE profit (
    id INTEGER PRIMARY KEY NOT NULL,
    user_id INTEGER NOT NULL,
    profit DECIMAL NOT NULL,
    risk_to_reward DECIMAL NOT NULL,
    creation_date VARCHAR(10) NOT NULL
);