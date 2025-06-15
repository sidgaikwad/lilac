-- services table
CREATE TABLE services (
    service_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    service_name text NOT NULL,
    organization_id uuid NOT NULL REFERENCES organizations(organization_id),
    service_type text NOT NULL,
    service_configuration json NOT NULL DEFAULT '{}'::json,
    created_at timestamptz NOT NULL DEFAULT (now() at time zone 'UTC'),
    updated_at timestamptz NOT NULL DEFAULT (now() at time zone 'UTC'),
    deleted_at timestamptz
);
CREATE INDEX idx_services_organization_id ON services(organization_id);
CREATE TRIGGER update_services_updated_at
    BEFORE UPDATE
    ON
        services
    FOR EACH ROW
EXECUTE PROCEDURE set_updated_at_now();