-- Remove dataset_path column from pipeline_jobs table
ALTER TABLE pipeline_jobs
DROP COLUMN dataset_path;