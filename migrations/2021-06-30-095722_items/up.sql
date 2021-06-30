-- Your SQL goes here

CREATE TABLE items (
    item_id INTEGER PRIMARY KEY,
    name character varying(50) NOT NULL,
    bucket_id INTEGER,
    date_created timestamp without time zone NOT NULL,
    date_modified timestamp without time zone NOT NULL,
    completed boolean DEFAULT false
);