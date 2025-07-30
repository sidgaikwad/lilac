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

CREATE TYPE node_status AS ENUM ('available', 'busy');
CREATE TYPE cpu_configuration AS (
    manufacturer text,
    architecture text,
    millicores integer
);
CREATE TYPE gpu_configuration AS (
    manufacturer text,
    model_name text,
    memory_mb integer,
    count integer
);

CREATE TABLE cluster_nodes (
    node_id uuid PRIMARY KEY,
    cluster_id uuid NOT NULL REFERENCES clusters(cluster_id),
    node_status node_status NOT NULL,
    heartbeat_timestamp timestamptz NOT NULL,
    memory_mb integer NOT NULL,
    cpu cpu_configuration NOT NULL,
    gpu gpu_configuration,
    created_at timestamptz NOT NULL DEFAULT (now() at time zone 'UTC'),
    updated_at timestamptz NOT NULL DEFAULT (now() at time zone 'UTC'),
    deleted_at timestamptz
    -- Add Assigned Job ID
);
CREATE TRIGGER update_cluster_nodes_updated_at
    BEFORE UPDATE
    ON
        cluster_nodes
    FOR EACH ROW
EXECUTE PROCEDURE set_updated_at_now();