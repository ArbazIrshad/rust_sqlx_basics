-- Add migration script here
CREATE TABLE IF NOT EXISTS second_test (
    id INTEGER PRIMARY KEY NOT NULL,
    description TEXT NOT NULL,
    done BOOLEAN NOT NULL
);