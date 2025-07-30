-- Remove the single_owner_check constraint
ALTER TABLE api_keys
DROP CONSTRAINT single_owner_check;
