-- functions
CREATE  FUNCTION set_updated_at_now()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- user tables
CREATE TABLE users (
    user_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    email text NOT NULL UNIQUE,
    email_verified boolean NOT NULL,
    password_hash text NOT NULL,
    created_at timestamptz NOT NULL DEFAULT (now() at time zone 'UTC'),
    updated_at timestamptz NOT NULL DEFAULT (now() at time zone 'UTC'),
    deleted_at timestamptz
);
CREATE TRIGGER update_users_updated_at
    BEFORE UPDATE
    ON
        users
    FOR EACH ROW
EXECUTE PROCEDURE set_updated_at_now();

-- organization tables
CREATE TABLE organizations (
    organization_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_name text NOT NULL,
    created_at timestamptz NOT NULL DEFAULT (now() at time zone 'UTC'),
    updated_at timestamptz NOT NULL DEFAULT (now() at time zone 'UTC'),
    deleted_at timestamptz
);
CREATE TABLE organization_memberships (
    organization_id uuid NOT NULL REFERENCES organizations(organization_id),
    user_id uuid NOT NULL REFERENCES users(user_id),
    role text NOT NULL DEFAULT 'member' CHECK (role IN ('owner', 'admin', 'member')),
    joined_at timestamptz NOT NULL DEFAULT (now() at time zone 'UTC'),
    PRIMARY KEY (organization_id, user_id)
);
CREATE INDEX idx_organization_memberships_user_id ON organization_memberships(user_id);
CREATE TRIGGER update_organizations_updated_at
    BEFORE UPDATE
    ON
        organizations
    FOR EACH ROW
EXECUTE PROCEDURE set_updated_at_now();

-- project tables
CREATE TABLE projects (
    project_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    project_name text NOT NULL,
    organization_id uuid NOT NULL REFERENCES organizations(organization_id),
    created_at timestamptz NOT NULL DEFAULT (now() at time zone 'UTC'),
    updated_at timestamptz NOT NULL DEFAULT (now() at time zone 'UTC'),
    deleted_at timestamptz
);
CREATE INDEX idx_projects_organization_id ON projects(organization_id);
CREATE TRIGGER update_projects_updated_at
    BEFORE UPDATE
    ON
        projects
    FOR EACH ROW
EXECUTE PROCEDURE set_updated_at_now();

-- datasets tables
CREATE TABLE datasets (
    dataset_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    dataset_name text NOT NULL,
    description text,
    project_id uuid NOT NULL REFERENCES projects(project_id),
    dataset_path text NOT NULL,
    created_at timestamptz NOT NULL default (now() at time zone 'UTC'),
    updated_at timestamptz NOT NULL default (now() at time zone 'UTC'),
    deleted_at timestamptz
);
CREATE INDEX idx_datasets_project_id ON datasets(project_id);
CREATE TRIGGER update_datasets_updated_at
    BEFORE UPDATE
    ON
        datasets
    FOR EACH ROW
EXECUTE PROCEDURE set_updated_at_now();

-- pipeline tables
CREATE TABLE pipelines (
    pipeline_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    pipeline_name text NOT NULL,
    description text,
    project_id uuid NOT NULL REFERENCES projects(project_id),
    created_at timestamptz NOT NULL default (now() at time zone 'UTC'),
    updated_at timestamptz NOT NULL default (now() at time zone 'UTC'),
    deleted_at timestamptz
);
CREATE INDEX idx_pipelines_project_id ON pipelines(project_id);
CREATE TRIGGER update_pipelines_updated_at
    BEFORE UPDATE
    ON
        pipelines
    FOR EACH ROW
EXECUTE PROCEDURE set_updated_at_now();

-- step definitions table
CREATE TABLE step_definitions (
    step_definition_id uuid PRIMARY KEY,
    name text NOT NULL,
    description text,
    category text NOT NULL,
    inputs text[] NOT NULL DEFAULT array[]::text[],
    outputs text[] NOT NULL DEFAULT array[]::text[],
    step_type text NOT NULL UNIQUE,
    schema jsonb NOT NULL DEFAULT '{}'::jsonb
);

-- step tables
CREATE TABLE steps (
    step_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    pipeline_id uuid NOT NULL REFERENCES pipelines(pipeline_id),
    step_definition_id uuid NOT NULL REFERENCES step_definitions(step_definition_id),
    step_parameters jsonb NOT NULL DEFAULT '{}'::jsonb,
    created_at timestamptz NOT NULL DEFAULT (now() at time zone 'UTC'),
    updated_at timestamptz NOT NULL DEFAULT (now() at time zone 'UTC'),
    deleted_at timestamptz
);
CREATE INDEX idx_steps_pipeline_id ON steps (pipeline_id);
CREATE TRIGGER update_steps_updated_at
    BEFORE UPDATE
    ON
        steps
    FOR EACH ROW
EXECUTE PROCEDURE set_updated_at_now();

CREATE TABLE step_connections (
    pipeline_id uuid NOT NULL REFERENCES pipelines(pipeline_id),
    from_step_id uuid NOT NULL REFERENCES steps(step_id),
    to_step_id uuid NOT NULL REFERENCES steps(step_id),
    PRIMARY KEY (from_step_id, to_step_id)
);
CREATE INDEX idx_step_connections_pipeline_id ON steps (pipeline_id);

-- jobs table
CREATE TABLE pipeline_jobs (
    job_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    pipeline_id uuid NOT NULL REFERENCES pipelines(pipeline_id),
    input_dataset_id uuid NOT NULL REFERENCES datasets(dataset_id),
    status text NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'in_progress', 'completed', 'failed')),
    created_at timestamptz NOT NULL DEFAULT (now() at time zone 'UTC'),
    started_at timestamptz,
    ended_at timestamptz
);
CREATE INDEX idx_pipeline_jobs_status ON pipeline_jobs(status);
