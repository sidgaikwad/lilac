CREATE TABLE organization_memberships (
    organization_id uuid NOT NULL REFERENCES organizations(organization_id),
    user_id uuid NOT NULL REFERENCES users(user_id),
    joined_at timestamptz NOT NULL default (now() at time zone 'UTC'),
    PRIMARY KEY (organization_id, user_id)
);
