CREATE TABLE pipelines (
    pipeline_id uuid PRIMARY KEY,
    pipeline_name text NOT NULL,
    description text,
    organization_id uuid NOT NULL REFERENCES organizations(organization_id),
    created_at timestamptz NOT NULL default (now() at time zone 'UTC')
);
CREATE INDEX idx_pipelines_organization_id ON pipelines(organization_id);
