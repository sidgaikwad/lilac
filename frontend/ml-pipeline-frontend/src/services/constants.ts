export const BASE_URL =
  import.meta.env.VITE_LILAC_API_ENDPOINT || 'http://localhost:8081';
export const API_URL = BASE_URL;

export enum QueryKeys {
  LOGIN = 'Login',
  SIGN_UP = 'SignUp',
  GET_ACCOUNT_DETAILS = 'GetAccountDetails',

  GET_USER = 'GetUser',


  GET_PROJECT = 'GetProject',
  CREATE_PROJECT = 'CreateProject',
  UPDATE_PROJECT = 'UpdateProject',
  LIST_PROJECTS = 'ListProjects',
  DELETE_PROJECT = 'DeleteProject',

  CONNECT_AWS_INTEGRATION = 'ConnectAwsIntegration',

  GET_DATASET = 'GetDataset',
  CREATE_DATASET = 'CreateDataset',
  LIST_DATASETS = 'ListDatasets',
  LIST_DATASET_S3_OBJECTS = 'ListDatsetS3Objects',
}
