-- Your SQL goes here

CREATE TABLE users (
    id SERIAL PRIMARY KEY NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL
);

CREATE TABLE jobs (
    id SERIAL PRIMARY KEY NOT NULL,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE ,
    schedule VARCHAR NOT NULL,
    command VARCHAR NOT NULL,
    last_run INTEGER NOT NULL,
    next_run INTEGER NOT NULL
);

CREATE TABLE secrets (
    id SERIAL PRIMARY KEY NOT NULL,
    job_id INTEGER NOT NULL REFERENCES jobs(id) ON DELETE CASCADE,
    key TEXT NOT NULL,
    value TEXT NOT NULL
);

INSERT INTO users(email, password) VALUES ('max@max.de', 'Heroes');
INSERT INTO jobs(user_id, schedule, command, last_run, next_run) VALUES (1, ('* * * * *'), 'echo $hello >> world.txt', 0, 0);
INSERT INTO secrets(job_id, key, value) VALUES (1, 'hello', 'world')
