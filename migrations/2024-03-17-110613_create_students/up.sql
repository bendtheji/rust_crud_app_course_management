-- Your SQL goes here
CREATE TABLE students (
    id SERIAL PRIMARY KEY,
    email VARCHAR NOT NULL,
    created_at timestamp,
    updated_at timestamp,
    UNIQUE(email)
);