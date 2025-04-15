CREATE TABLE pipeline_jobs (
    job_id uuid PRIMARY KEY,
    pipeline_id uuid NOT NULL REFERENCES pipelines(pipeline_id),
    status text NOT NULL DEFAULT 'PENDING',
    created_at timestamptz NOT NULL DEFAULT (now() at time zone 'UTC'),
    started_at timestamptz,
    ended_at timestamptz
);
CREATE INDEX idx_pipeline_jobs_status ON pipeline_jobs(status);
