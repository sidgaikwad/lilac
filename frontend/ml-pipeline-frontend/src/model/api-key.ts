export type ApiKey = {
  id: string;
  prefix: string;
  created_at: string;
  last_used_at: string | null;
  expires_at: string | null;
};

export type NewApiKey = {
  id: string;
  prefix: string;
  created_at: string;
  key: string;
};
