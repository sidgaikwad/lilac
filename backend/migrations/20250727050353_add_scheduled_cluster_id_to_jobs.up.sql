-- Add the column to store which cluster a job was scheduled on
ALTER TABLE training_jobs
ADD COLUMN scheduled_cluster_id UUID REFERENCES clusters(cluster_id);