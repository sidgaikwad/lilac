export const API_URL =
  import.meta.env.VITE_LILAC_API_ENDPOINT || 'http://localhost:8081';

export enum QueryKeys {
  LOGIN = 'Login',
  SIGN_UP = 'SignUp',
  GET_ACCOUNT_DETAILS = 'GetAccountDetails',

  GET_USER = 'GetUser',

  GET_JOB = 'GetJob',
  LIST_JOBS = 'ListJobs',

  GET_PROJECT = 'GetProject',
  CREATE_PROJECT = 'CreateProject',
  UPDATE_PROJECT = 'UpdateProject',
  LIST_PROJECTS = 'ListProjects',
  DELETE_PROJECT = 'DeleteProject',

  LIST_SERVICES = 'ListServices',
  CREATE_SERVICE = 'CreateService',

  GET_CREDENTIAL = 'GetCredential',
  CREATE_CREDENTIAL = 'CreateCredential',
  LIST_CREDENTIALS = 'ListCredentials',
  DELETE_CREDENTIAL = 'DeleteCredential',

  LIST_API_KEYS = 'ListApiKeys',
  CREATE_API_KEY = 'CreateApiKey',
  DELETE_API_KEY = 'DeleteApiKey',

  GET_QUEUE = 'GetQueue',
  LIST_QUEUES = 'ListQueues',
  LIST_QUEUE_JOBS = 'ListQueueJobs',
  CREATE_QUEUE = 'CreateQueue',
  DELETE_QUEUE = 'DeleteQueue',

  GET_CLUSTER = 'GetCluster',
  GET_CLUSTER_INFO = 'GetClusterInfo',
  GET_CLUSTER_NODE = 'GetClusterNode',
  LIST_CLUSTER_JOBS = 'ListClusterJobs',
  LIST_CLUSTER_KEYS = 'ListClusterKeys',
  LIST_CLUSTER_NODES = 'ListClusterNodes',
  CREATE_CLUSTER = 'CreateCluster',
  CREATE_CLUSTER_KEY = 'CreateClusterKey',
  LIST_CLUSTERS = 'ListClusters',
  DELETE_CLUSTER = 'DeleteCluster',
  TEST_CLUSTER_CONNECTION = 'TestClusterConnection',

  CONNECT_AWS_INTEGRATION = 'ConnectAwsIntegration',

  GET_DATASET = 'GetDataset',
  CREATE_DATASET = 'CreateDataset',
  TEST_DATASET_CONNECTION = 'TestDatasetConnection',
  LIST_DATASETS = 'ListDatasets',
  LIST_DATASET_S3_OBJECTS = 'ListDatsetS3Objects',
  LIST_WORKSPACES = 'ListWorkspaces',
  CREATE_WORKSPACE = 'CreateWorkspace',

  CANCEL_TRAINING_JOB = 'CancelTrainingJob',
}
