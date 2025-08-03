CREATE TABLE queues (
    queue_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL UNIQUE,
    priority INTEGER NOT NULL
);

CREATE TABLE queue_cluster_assignments (
    queue_id UUID NOT NULL REFERENCES queues(queue_id) ON DELETE CASCADE,
    cluster_id UUID NOT NULL REFERENCES clusters(cluster_id) ON DELETE CASCADE,
    "order" INTEGER NOT NULL,
    PRIMARY KEY (queue_id, cluster_id),
    UNIQUE (queue_id, "order")
);

ALTER TABLE training_jobs
ADD COLUMN queue_id UUID NOT NULL REFERENCES queues(queue_id),
DROP COLUMN priority;