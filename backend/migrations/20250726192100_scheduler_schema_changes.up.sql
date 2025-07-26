-- Add platform_type to clusters table
ALTER TABLE clusters
ADD COLUMN platform_type VARCHAR(255) NOT NULL DEFAULT 'kubernetes';

-- Add priority and resource_requirements to training_jobs table
ALTER TABLE training_jobs
ADD COLUMN priority INTEGER NOT NULL DEFAULT 100,
ADD COLUMN resource_requirements JSONB NOT NULL;

-- Remove cluster_id from training_jobs table
ALTER TABLE training_jobs
DROP COLUMN cluster_id;

-- Create the new training_job_cluster_targets join table
CREATE TABLE training_job_cluster_targets (
    job_id UUID NOT NULL REFERENCES training_jobs(id) ON DELETE CASCADE,
    cluster_id UUID NOT NULL REFERENCES clusters(cluster_id) ON DELETE CASCADE,
    priority INTEGER NOT NULL,
    PRIMARY KEY (job_id, cluster_id)
);