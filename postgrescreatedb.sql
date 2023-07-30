CREATE DATABASE rust_server;
\c rust_server;
ALTER ROLE postgres WITH PASSWORD 'postgres123';
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  user_name VARCHAR NOT NULL,
  user_email VARCHAR NOT NULL,
  user_password VARCHAR NOT NULL
);
