-- Add back cluster_id to training_jobs table
-- Note: This will not restore the original data, but it reverts the schema.
ALTER TABLE training_jobs
ADD COLUMN cluster_id UUID;

-- Add a foreign key constraint to the re-added column
-- This assumes the clusters table and its primary key exist.
ALTER TABLE training_jobs
ADD CONSTRAINT fk_cluster
FOREIGN KEY (cluster_id)
REFERENCES clusters(id);

-- Remove priority and resource_requirements from training_jobs table
ALTER TABLE training_jobs
DROP COLUMN priority,
DROP COLUMN resource_requirements;
