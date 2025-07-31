export interface Cluster {
  clusterId: string;
  clusterName: string;
  clusterDescription?: string;
}

export interface ClusterSummary {
  clusterId: string;
  clusterName: string;
  clusterDescription?: string;
}

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

export interface ClusterInfo {
  clusterId: string;
  clusterName: string;
  clusterDescription?: string;
  totalNodes: number;
  busyNodes: number;
  memoryInfo: {
    totalMemoryMb: number;
    usedMemoryMb: number;
  };
  cpuInfo: {
    totalMillicores: number;
    usedMillicores: number;
  };
  gpuInfo: {
    totalGpus: number;
    usedGpus: number;
  };
  jobInfo: {
    totalRunningJobs: number;
  };
  createdAt: string;
  updatedAt: string;
}

export interface ClusterJob {
  jobId: string;
  jobName: string;
  jobStatus: string;
  nodeId?: string;
  queueId: string;
  resourceRequirements: ResourceRequirements;
  createdAt: string;
  updatedAt: string;
}
