-- Add up migration script here
CREATE TABLE clusters (
    cluster_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    cluster_name text NOT NULL,
    cluster_description text,
    cluster_config jsonb NOT NULL DEFAULT '{}'::jsonb,
    created_at timestamptz NOT NULL DEFAULT (now() at time zone 'UTC'),
    updated_at timestamptz NOT NULL DEFAULT (now() at time zone 'UTC'),
    deleted_at timestamptz
);
CREATE TRIGGER update_clusters_updated_at
    BEFORE UPDATE
    ON
        clusters
    FOR EACH ROW
EXECUTE PROCEDURE set_updated_at_now();