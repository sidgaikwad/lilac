CREATE TYPE training_job_status AS ENUM (
    'queued',
    'starting',
    'running',
    'succeeded',
    'failed'
);

CREATE TABLE training_jobs (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    definition TEXT NOT NULL,
    status training_job_status NOT NULL,
    cluster_id UUID NOT NULL,
    instance_id UUID,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);