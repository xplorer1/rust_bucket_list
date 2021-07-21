-- Your SQL goes here

CREATE TABLE buckets (
    bucket_id INTEGER PRIMARY KEY,
    name character varying(255) NOT NULL,
    date_created timestamp without time zone NOT NULL,
    date_modified timestamp without time zone NOT NULL,
    user_id integer
);