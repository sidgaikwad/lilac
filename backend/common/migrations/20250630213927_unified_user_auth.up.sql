-- Add oidc_provider and oidc_provider_id columns to the users table
ALTER TABLE "users" ADD COLUMN oidc_provider TEXT;
ALTER TABLE "users" ADD COLUMN oidc_provider_id TEXT;

-- Make the password_hash column nullable
ALTER TABLE "users" ALTER COLUMN password_hash DROP NOT NULL;
