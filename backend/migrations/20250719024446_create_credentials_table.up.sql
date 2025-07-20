CREATE TABLE credentials (
    credential_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    credential_name text NOT NULL,
    credential_description text,
    credentials jsonb NOT NULL DEFAULT '{}'::jsonb,
    created_at timestamptz NOT NULL DEFAULT (now() at time zone 'UTC'),
    updated_at timestamptz NOT NULL DEFAULT (now() at time zone 'UTC'),
    deleted_at timestamptz
);
CREATE TRIGGER update_clusters_updated_at
    BEFORE UPDATE
    ON
        credentials
    FOR EACH ROW
EXECUTE PROCEDURE set_updated_at_now();