export interface Dataset {
  id: string;
  name: string;
  description?: string;
  projectId: string;
}

export interface DatasetSummary {
  id: string;
  name: string;
  description?: string;
}
