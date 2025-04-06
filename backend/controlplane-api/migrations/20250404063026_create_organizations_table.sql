CREATE TABLE organizations (
    organization_id uuid PRIMARY KEY,
    organization_name text NOT NULL,
    created_at timestamptz NOT NULL default (now() at time zone 'UTC')
);