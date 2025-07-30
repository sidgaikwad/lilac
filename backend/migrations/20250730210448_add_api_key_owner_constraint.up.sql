-- Add a CHECK constraint to ensure that either user_id or cluster_id is set, but not both.
ALTER TABLE api_keys
ADD CONSTRAINT single_owner_check
CHECK (
    (user_id IS NOT NULL AND cluster_id IS NULL) OR
    (user_id IS NULL AND cluster_id IS NOT NULL)
);
