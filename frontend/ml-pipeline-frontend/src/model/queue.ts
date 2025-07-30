export type Queue = {
  id: string;
  name: string;
  priority: number;
  cluster_targets: string[];
};