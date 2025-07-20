import { AwsCredentials } from '@/services/credentials';

export interface Credential {
  credentialId: string;
  credentialName: string;
  credentialDescription?: string;
  credentials: AwsCredentials;
}

export interface CredentialSummary {
  credentialId: string;
  credentialName: string;
  credentialDescription?: string;
  credentialType: string;
}
