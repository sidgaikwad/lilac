export interface S3Source {
  sourceType: 'S3';
  bucketName: string;
  accessKey: string;
  secretKey: string;
}

export interface SnowflakeSource {
  sourceType: 'Snowflake';
  username: string;
  password: string;
  account: string;
  warehouse?: string;
  database?: string;
  schema?: string;
  role?: string;
}
