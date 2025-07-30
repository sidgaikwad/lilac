-- Add priority and resource_requirements to training_jobs table
ALTER TABLE training_jobs
ADD COLUMN priority INTEGER NOT NULL DEFAULT 100,
ADD COLUMN resource_requirements JSONB NOT NULL;

