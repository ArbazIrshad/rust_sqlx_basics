-- Add migration script here
-- CREATE TABLE IF NOT EXISTS tasks
-- (
--     id INT PRIMARY KEY NOT NULL,
--     title TEXT NOT NULL,
--     body TEXT NOT NULL,
--     isDone Boolean Not NUll,
-- );
CREATE TABLE IF NOT EXISTS todos
(
    id          INTEGER PRIMARY KEY NOT NULL,
    description TEXT                NOT NULL,
    done        BOOLEAN             NOT NULL DEFAULT 0
);