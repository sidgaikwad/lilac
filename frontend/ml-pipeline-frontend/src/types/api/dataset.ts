import { S3Source, SnowflakeSource } from '@/services';

export interface Dataset {
  id: string;
  name: string;
  description?: string;
  projectId: string;
  datasetSource: S3Source | SnowflakeSource;
}

export interface DatasetSummary {
  id: string;
  name: string;
  description?: string;
  datasetSource: string;
}
