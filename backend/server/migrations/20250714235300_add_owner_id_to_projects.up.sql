-- Add a nullable owner_id column to projects
ALTER TABLE projects
ADD COLUMN owner_id UUID REFERENCES users(user_id);

-- Backfill owner_id from the project_memberships table for existing projects
UPDATE projects p
SET owner_id = (
    SELECT user_id
    FROM project_memberships pm
    WHERE pm.project_id = p.project_id
    AND pm.role = 'admin'
    LIMIT 1
);

-- Set the owner_id column to be NOT NULL
ALTER TABLE projects
ALTER COLUMN owner_id SET NOT NULL;