ALTER TABLE training_jobs DROP CONSTRAINT training_jobs_queue_id_fkey;
ALTER TABLE training_jobs ALTER COLUMN queue_id DROP NOT NULL;
ALTER TABLE training_jobs
ADD CONSTRAINT training_jobs_queue_id_fkey
FOREIGN KEY (queue_id)
REFERENCES queues(queue_id)
ON DELETE SET NULL;