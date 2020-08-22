-- Your SQL goes here

CREATE TABLE users (
  id VARCHAR(36) PRIMARY KEY,
  email VARCHAR NOT NULL,
  pw_hash VARCHAR NOT NULL,
  created_at TIMESTAMPTZ DEFAULT now() at time zone 'utc'
);
