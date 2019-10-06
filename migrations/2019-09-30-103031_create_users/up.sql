-- Your SQL goes here
CREATE TABLE users (
  id UUID NOT NULL PRIMARY KEY, -- uuid v4
  email VARCHAR(100) NOT NULL,
  hash VARCHAR(122) NOT NULL, --argon2 hash
  first_name VARCHAR(50) NOT NULL,
  last_name VARCHAR(50) NOT NULL,
  created_at TIMESTAMP NOT NULL,
  yapily_id VARCHAR NOT NULL -- From yapily
);
