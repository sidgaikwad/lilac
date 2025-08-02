export enum Routes {
  HOME = '/',
  LOGIN = '/login',
  LOGOUT = '/logout',
  SIGNUP = '/signup',
  ACCOUNT_SETTINGS = '/account/settings',
  API_KEYS = '/account/api-keys',

  DATA_SOURCES = '/data-sources',
  DATA_SOURCE_DETAILS = '/data-sources/:dataSourceId',
  CLUSTERS = '/clusters',
  CLUSTER_DETAILS = '/clusters/:clusterId',
  QUEUES = '/queues',
  QUEUE_DETAILS = '/queues/:queueId',
  ORG_SETTINGS = '/organization/settings',

  PROJECTS = '/projects',
  PROJECT_DETAILS = '/projects/:projectId',
  PROJECT_SETTINGS = '/projects/:projectId/settings',
  PROJECT_EXPERIMENTS = '/projects/:projectId/experiments',
  PROJECT_WORKSPACES = '/projects/:projectId/workspaces',
}
