export interface Workspace {
  id: string;
  name: string;
  status: 'Running' | 'Stopped' | 'Starting' | 'Error';
  environment: {
    name: string;
    icon: string; // URL or identifier for an icon like VSCode, Jupyter
  };
  hardware: {
    tier: string; // e.g., "Small"
    spec: string; // e.g., "2 Core, 4 GiB RAM"
    cpu: number;
    memory: number;
  };
  lastStarted: string; // ISO date string
}

export const mockWorkspaces: Workspace[] = [
  {
    id: 'ws-1',
    name: 'feature-extraction-notebook',
    status: 'Running',
    environment: { name: 'JupyterLab', icon: 'jupyter-icon' },
    hardware: { tier: 'Medium', spec: '4 Core, 8 GiB RAM', cpu: 4, memory: 8 },
    lastStarted: '2025-07-15T21:10:00Z',
  },
  {
    id: 'ws-2',
    name: 'andrea_lowe_s_sas_viya_session',
    status: 'Stopped',
    environment: { name: 'SAS Viya', icon: 'sas-icon' },
    hardware: { tier: 'Small', spec: '2 Core, 4 GiB RAM', cpu: 2, memory: 4 },
    lastStarted: '2025-07-14T18:45:00Z',
  },
];

export const mockEnvironments = [
  {
    name: 'JupyterLab',
    icon: 'jupyter-icon',
    description:
      'A web-based interactive development environment for notebooks, code, and data.',
  },
  {
    name: 'VSCode',
    icon: 'vscode-icon',
    description:
      'A lightweight but powerful source code editor that runs on your desktop.',
  },
  {
    name: 'RStudio',
    icon: 'rstudio-icon',
    description: 'An integrated development environment for R and Python.',
  },
];

export const mockWorkspaceHosts = [
  {
    name: 'AWS Kubernetes',
    icon: 'aws-icon',
    description: 'Run on a managed Kubernetes cluster in AWS.',
  },
  {
    name: 'GCP Kubernetes',
    icon: 'gcp-icon',
    description: 'Run on a managed Kubernetes cluster in GCP.',
  },
  {
    name: 'Slurm',
    icon: 'slurm-icon',
    description: 'Run on a Slurm cluster.',
  },
];

export const mockComputeClusters = [
  {
    name: 'None',
    icon: 'none-icon',
    description: '',
  },
  {
    name: 'Ray',
    icon: 'ray-icon',
    description: '',
  },
];
