-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email VARCHAR(100) NOT NULL,
  hash VARCHAR(122) NOT NULL, --argon2 hash
  first_name VARCHAR NOT NULL,
  last_name VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL
);
