export const BASE_URL = 'http://localhost:3000';
export const API_URL = BASE_URL;

export enum QueryKeys {
  LOGIN = 'Login',
  SIGN_UP = 'SignUp',
  GET_ACCOUNT_DETAILS = 'GetAccountDetails',

  GET_USER = 'GetUser',

  GET_ORGANIZATION = 'GetOrganization',
  CREATE_ORGANIZATION = 'CreateOrganization',
  UPDATE_ORGANIZATION = 'UpdateOrganization',
  LIST_ORGANIZATIONS = 'ListOrganizations',
  DELETE_ORGANIZATION = 'DeleteOrganization',

  GET_PROJECT = 'GetProject',
  CREATE_PROJECT = 'CreateProject',
  UPDATE_PROJECT = 'UpdateProject',
  LIST_PROJECTS = 'ListProjects',
  DELETE_PROJECT = 'DeleteProject',

  CONNECT_AWS_INTEGRATION = 'ConnectAwsIntegration',

  GET_PIPELINE = 'GetPipeline',
  CREATE_PIPELINE = 'CreatePipeline',
  UPDATE_PIPELINE = 'UpdatePipeline',
  LIST_PIPELINES = 'ListPipelines',
  DELETE_PIPELINE = 'DeletePipeline',
  RUN_PIPELINE = 'RunPipeline',

  GET_DATASET = 'GetDataset',
  CREATE_DATASET = 'CreateDataset',
  LIST_DATASETS = 'ListDatasets',
  LIST_DATASET_S3_OBJECTS = 'ListDatsetS3Objects',

  LIST_JOBS = 'ListJobs',

  LIST_STEP_DEFINITIONS = 'ListStepDefinitions',
}
