import { ResourceRequirements } from './cluster';

export interface QueueJob {
  jobId: string;
  jobName: string;
  jobStatus: string;
  nodeId?: string;
  queueId: string;
  resourceRequirements: ResourceRequirements;
  createdAt: string;
  updatedAt: string;
}

export type Queue = {
  id: string;
  name: string;
  priority: number;
  clusterTargets: string[];
};
