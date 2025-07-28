-- Revert the changes made to the `training_jobs` table.
ALTER TABLE training_jobs
DROP COLUMN queue_id,
ADD COLUMN priority INTEGER NOT NULL DEFAULT 100;

-- Drop the join table for queue-to-cluster assignments.
DROP TABLE queue_cluster_assignments;

-- Drop the `queues` table.
DROP TABLE queues;