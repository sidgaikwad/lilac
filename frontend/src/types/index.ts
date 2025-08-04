export * from './api';

export type Provider = {
  name: string;
  type: 'oidc' | 'oauth2';
};
