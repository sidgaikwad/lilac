export type ApiKey = {
  id: string;
  prefix: string;
  createdAt: string;
  lastUsedAt: string | null;
  expiresAt: string | null;
};

export type NewApiKey = {
  id: string;
  prefix: string;
  createdAt: string;
  key: string;
};
