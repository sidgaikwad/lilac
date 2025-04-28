import { Step } from './steps';

export interface PipelineSummary {
  id: string;
  name: string;
  description?: string;
}

export interface Pipeline {
  id: string;
  name: string;
  description?: string;
  projectId: string;
  steps: Step[];
  stepConnections: [string, string][];
}
