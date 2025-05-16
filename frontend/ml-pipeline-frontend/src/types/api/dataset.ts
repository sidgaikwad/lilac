export interface Dataset {
  id: string;
  name: string;
  description?: string;
  projectId: string;
  files: {
    fileName: string;
    fileType: string;
    size: number;
    createdAt: string;
    url: string;
  }[];
}

export interface DatasetSummary {
  id: string;
  name: string;
  description?: string;
}
