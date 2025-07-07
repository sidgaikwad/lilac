-- functions
CREATE  FUNCTION set_updated_at_now()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- user tables
CREATE TYPE auth_provider AS ENUM (
    'email',
    'google',
    'github',
    'gitlab',
    'ldap',
    'oidc'
);

CREATE TABLE users (
    user_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    email text NOT NULL UNIQUE,
    email_verified boolean NOT NULL,
    password_hash text,
    login_method auth_provider,
    sso_provider_id TEXT,
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

-- project tables
CREATE TABLE projects (
    project_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    project_name text NOT NULL,
    aws_integration jsonb,
    created_at timestamptz NOT NULL DEFAULT (now() at time zone 'UTC'),
    updated_at timestamptz NOT NULL DEFAULT (now() at time zone 'UTC'),
    deleted_at timestamptz
);
CREATE TRIGGER update_projects_updated_at
    BEFORE UPDATE
    ON
        projects
    FOR EACH ROW
EXECUTE PROCEDURE set_updated_at_now();

-- project_memberships table
CREATE TABLE project_memberships (
    project_id uuid NOT NULL REFERENCES projects(project_id) ON DELETE CASCADE,
    user_id uuid NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    role text,
    PRIMARY KEY (project_id, user_id)
);
CREATE INDEX idx_project_memberships_user_id ON project_memberships(user_id);
CREATE INDEX idx_project_memberships_project_id ON project_memberships(project_id);

-- datasets tables
CREATE TABLE datasets (
    dataset_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    dataset_name text NOT NULL,
    description text,
    project_id uuid NOT NULL REFERENCES projects(project_id),
    dataset_source jsonb NOT NULL DEFAULT '{}'::jsonb,
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

-- services table
CREATE TABLE services (
    service_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    service_name text NOT NULL,
    project_id uuid NOT NULL REFERENCES projects(project_id),
    service_type text NOT NULL,
    service_configuration json NOT NULL DEFAULT '{}'::json,
    created_at timestamptz NOT NULL DEFAULT (now() at time zone 'UTC'),
    updated_at timestamptz NOT NULL DEFAULT (now() at time zone 'UTC'),
    deleted_at timestamptz
);
CREATE INDEX idx_services_project_id ON services(project_id);
CREATE TRIGGER update_services_updated_at
    BEFORE UPDATE
    ON
        services
    FOR EACH ROW
EXECUTE PROCEDURE set_updated_at_now();
