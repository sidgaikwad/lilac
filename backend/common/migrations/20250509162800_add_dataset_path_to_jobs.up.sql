-- Add dataset_path column to pipeline_jobs table
ALTER TABLE pipeline_jobs
ADD COLUMN dataset_path VARCHAR(255) NULL;