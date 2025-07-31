export interface CreateWorkspaceRequest {
  name: string;
  cluster_id: string;
  ide: 'vscode' | 'jupyterlab' | 'rstudio';
  image: string;
  cpu_millicores: number;
  memory_mb: number;
  gpu?: boolean;
}

export interface Workspace {
  id: string;
  name: string;
  project_id: string;
  owner_id: string;
  cluster_id: string;
  ide: 'vscode' | 'jupyterlab' | 'rstudio';
  image: string;
  cpu_millicores: number;
  memory_mb: number;
  status:
    | 'pending'
    | 'running'
    | 'stopping'
    | 'stopped'
    | 'failed'
    | 'terminated';
  url: string | null;
  gpu?: boolean;
}
