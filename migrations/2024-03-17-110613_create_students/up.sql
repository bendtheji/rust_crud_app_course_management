-- Your SQL goes here
CREATE TABLE students (
    id SERIAL PRIMARY KEY,
    email VARCHAR NOT NULL,
    UNIQUE(email)
);