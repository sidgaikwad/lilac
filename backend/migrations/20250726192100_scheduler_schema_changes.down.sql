-- Remove priority and resource_requirements from training_jobs table
ALTER TABLE training_jobs
DROP COLUMN priority,
DROP COLUMN resource_requirements;
