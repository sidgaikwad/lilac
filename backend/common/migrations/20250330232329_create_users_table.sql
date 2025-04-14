-- Add migration script here
CREATE TABLE users (
    user_id uuid PRIMARY KEY,
    email text NOT NULL,
    email_verified boolean NOT NULL,
    password_hash text NOT NULL,
    created_at timestamptz NOT NULL default (now() at time zone 'UTC')
);
CREATE INDEX idx_user_email ON users (email);