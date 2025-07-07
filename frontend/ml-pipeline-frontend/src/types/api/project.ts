export interface Project {
  id: string;
  name: string;
  awsIntegration?: {
    roleArn: string;
    externalId: string;
  };
}
