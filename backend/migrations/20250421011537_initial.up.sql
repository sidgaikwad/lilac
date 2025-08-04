-- functions
CREATE  FUNCTION set_updated_at_now()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- user tables
CREATE TYPE auth_provider AS ENUM (
    'password'
);
CREATE TABLE users (
    user_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    username text NOT NULL UNIQUE,
    first_name text,
    last_name text,
    password_hash text,
    login_method auth_provider,
    sso_provider_id TEXT,
    created_at timestamptz NOT NULL DEFAULT (now() at time zone 'UTC'),
    updated_at timestamptz NOT NULL DEFAULT (now() at time zone 'UTC'),
    deleted_at timestamptz
);
CREATE TRIGGER update_users_updated_at
    BEFORE UPDATE
    ON
        users
    FOR EACH ROW
EXECUTE PROCEDURE set_updated_at_now();

CREATE INDEX IF NOT EXISTS idx_users_username ON users (username);