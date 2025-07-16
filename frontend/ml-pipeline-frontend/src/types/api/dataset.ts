import { S3Source, SnowflakeSource } from '@/services';

export interface Dataset {
  datasetId: string;
  datasetName: string;
  description?: string;
  projectId: string;
  datasetSource: S3Source | SnowflakeSource;
}

export interface DatasetSummary {
  datasetId: string;
  datasetName: string;
  datasetDescription?: string;
  sourceType: string;
}
