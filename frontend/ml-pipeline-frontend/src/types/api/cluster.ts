import { KubernetesCluster } from '@/services/clusters';

export interface Cluster {
  clusterId: string;
  clusterName: string;
  clusterDescription?: string;
  clusterSource: KubernetesCluster;
}

export interface ClusterSummary {
  clusterId: string;
  clusterName: string;
  clusterDescription?: string;
  clusterType: string;
  infraType: string;
}
