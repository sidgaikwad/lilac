CREATE TYPE training_job_status AS ENUM (
    'queued',
    'starting',
    'running',
    'succeeded',
    'failed',
    'cancelled'
);

CREATE TABLE training_jobs (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    definition TEXT NOT NULL,
    status training_job_status NOT NULL,
    node_id UUID,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_training_jobs_node_id ON training_jobs (node_id);