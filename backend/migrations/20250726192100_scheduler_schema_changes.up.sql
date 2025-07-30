-- Add priority and resource_requirements to training_jobs table
ALTER TABLE training_jobs
ADD COLUMN priority INTEGER NOT NULL DEFAULT 100,
ADD COLUMN resource_requirements JSONB NOT NULL;

-- Remove cluster_id from training_jobs table
ALTER TABLE training_jobs
DROP COLUMN cluster_id;
