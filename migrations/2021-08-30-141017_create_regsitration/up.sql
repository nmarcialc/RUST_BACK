-- Your SQL goes here
CREATE TABLE registrations (
    id  SERIAL PRIMARY KEY,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    email TEXT NOT NULL,
    resort_id INTEGER NOT NULL
);
