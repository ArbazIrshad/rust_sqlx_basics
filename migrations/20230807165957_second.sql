-- Add migration script here
-- CREATE TABLE IF NOT EXISTS tasks
-- (
--     id INTEGER PRIMARY KEY NOT NULL,
--     title TEXT NOT NULL,
--     body TEXT NOT NULL,
--     done BOOLEAN NOT NULL DEFAULT 0,
-- );
CREATE TABLE IF NOT EXISTS tasks
(
    id          INTEGER PRIMARY KEY NOT NULL,
    title       TEXT                NOT NULL,
    body        TEXT                NOT NULL,
    done        BOOLEAN             NOT NULL DEFAULT 0
);