-- Create the new `queues` table to hold scheduling queues.
CREATE TABLE queues (
    queue_id UUID PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    priority INTEGER NOT NULL
);

-- Create the join table for queue-to-cluster assignments.
-- This defines the ordered list of clusters a queue can use.
CREATE TABLE queue_cluster_assignments (
    queue_id UUID NOT NULL REFERENCES queues(queue_id) ON DELETE CASCADE,
    cluster_id UUID NOT NULL REFERENCES clusters(cluster_id) ON DELETE CASCADE,
    "order" INTEGER NOT NULL,
    PRIMARY KEY (queue_id, cluster_id),
    UNIQUE (queue_id, "order")
);

-- Modify the `training_jobs` table to use the new queue system.
ALTER TABLE training_jobs
ADD COLUMN queue_id UUID NOT NULL REFERENCES queues(queue_id),
DROP COLUMN priority;