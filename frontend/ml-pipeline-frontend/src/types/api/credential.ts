import { AwsCredentials, GcpCredentials } from '@/services/credentials';

export interface Credential {
  credentialId: string;
  credentialName: string;
  credentialDescription?: string;
  credentials: AwsCredentials | GcpCredentials;
}

export interface CredentialSummary {
  credentialId: string;
  credentialName: string;
  credentialDescription?: string;
  credentialType: string;
}
