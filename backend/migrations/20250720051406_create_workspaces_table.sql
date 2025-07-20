-- Create workspaces table
CREATE TYPE ide_type AS ENUM ('vscode', 'jupyterlab', 'rstudio');
CREATE TYPE workspace_status AS ENUM ('pending', 'running', 'stopping', 'stopped', 'failed', 'terminated');

CREATE TABLE workspaces (
    workspace_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    workspace_name TEXT NOT NULL,
    project_id UUID NOT NULL,
    owner_id UUID NOT NULL,
    cluster_id UUID NOT NULL,
    ide ide_type NOT NULL,
    image TEXT NOT NULL,
    cpu_millicores INTEGER NOT NULL,
    memory_mb INTEGER NOT NULL,
    status workspace_status NOT NULL DEFAULT 'pending',
    url TEXT,
    token TEXT,
    public_key TEXT,
    private_key TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
