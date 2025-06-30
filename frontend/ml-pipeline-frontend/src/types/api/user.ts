export interface User {
  id: string;
  email: string;
  emailVerified: boolean;
  oidcProvider?: string;
  oidcProviderId?: string;
}
