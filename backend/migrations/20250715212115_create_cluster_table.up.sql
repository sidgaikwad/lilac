CREATE TABLE clusters (
    cluster_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    cluster_name text NOT NULL,
    cluster_description text,
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

CREATE TYPE cpu_manufacturer AS ENUM ('Intel', 'AMD', 'AWS');
CREATE TYPE architecture AS ENUM ('arm64', 'arm64-mac', 'i386', 'x86_64', 'x86_64-mac');
CREATE TYPE gpu_manufacturer AS ENUM ('Nvidia', 'AMD', 'Habana');
CREATE TYPE gpu_model AS ENUM (
    'Radeon Pro V520', 'Gaudi HL-205', 'A100', 'A10G', 'B200', 'H100',
    'H200', 'L4', 'L40S', 'T4', 'T4g', 'V100'
);

CREATE TYPE cpu_configuration AS (
    manufacturer cpu_manufacturer,
    architecture architecture,
    millicores integer
);
CREATE TYPE gpu_configuration AS (
    manufacturer gpu_manufacturer,
    model_name gpu_model,
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
    assigned_job_id uuid,
    reported_job_id uuid,
    created_at timestamptz NOT NULL DEFAULT (now() at time zone 'UTC'),
    updated_at timestamptz NOT NULL DEFAULT (now() at time zone 'UTC'),
    deleted_at timestamptz
);
CREATE TRIGGER update_cluster_nodes_updated_at
    BEFORE UPDATE
    ON
        cluster_nodes
    FOR EACH ROW
EXECUTE PROCEDURE set_updated_at_now();

CREATE INDEX IF NOT EXISTS idx_cluster_nodes_cluster_id ON cluster_nodes (cluster_id);