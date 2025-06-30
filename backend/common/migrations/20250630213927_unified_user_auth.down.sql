-- Revert the changes from the up migration
ALTER TABLE "users" DROP COLUMN oidc_provider;
ALTER TABLE "users" DROP COLUMN oidc_provider_id;
ALTER TABLE "users" ALTER COLUMN password_hash SET NOT NULL;
