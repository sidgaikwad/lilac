CREATE TYPE auth_provider AS ENUM (
    'email',
    'google',
    'github',
    'gitlab',
    'ldap',
    'oidc'
);
