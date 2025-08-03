export interface GpuRequirement {
  count: number;
  model?: string;
  memoryGb?: number;
}

export interface ResourceRequirements {
  cpuMillicores: number;
  memoryMb: number;
  gpus?: GpuRequirement;
}

export interface Job {
  jobId: string;
  jobName: string;
  jobStatus: 'queued' | 'starting' | 'running' | 'succeeded' | 'failed' | 'cancelled';
  nodeId?: string;
  queueId: string;
  resourceRequirements: ResourceRequirements;
  createdAt: string;
  updatedAt: string;
}