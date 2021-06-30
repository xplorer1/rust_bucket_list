-- Your SQL goes here

CREATE TABLE users (
    user_id INTEGER PRIMARY KEY,
    name character varying(50) NOT NULL,
    password character varying(50) NOT NULL,
    email character varying(355) NOT NULL,
    created_at TIMESTAMP NOT NULL
);