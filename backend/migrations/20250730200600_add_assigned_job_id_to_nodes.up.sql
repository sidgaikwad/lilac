-- The job assigned by the control plane's scheduler
ALTER TABLE cluster_nodes
ADD COLUMN assigned_job_id UUID REFERENCES training_jobs(id) ON DELETE SET NULL;

-- The job the agent reports it is currently running
ALTER TABLE cluster_nodes
ADD COLUMN reported_job_id UUID REFERENCES training_jobs(id) ON DELETE SET NULL;