export interface Project {
  id: string;
  name: string;
  organizationId: string;
  awsIntegration?: {
    roleArn: string;
    externalId: string;
  };
}
