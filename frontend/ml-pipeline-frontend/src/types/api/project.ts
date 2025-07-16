export interface Project {
  projectId: string;
  projectName: string;
  awsIntegration?: {
    roleArn: string;
    externalId: string;
  };
}
