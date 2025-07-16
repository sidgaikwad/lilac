export enum Routes {
  HOME = '/',
  LOGIN = '/login',
  LOGOUT = '/logout',
  SIGNUP = '/signup',
  ACCOUNT_SETTINGS = '/account/settings',

  DATA_SOURCES = '/data-sources',
  DATA_SOURCE_DETAILS = '/data-sources/:dataSourceId',
  CLUSTERS = '/clusters',
  CLUSTER_DETAILS = '/clusters/:clusterId',
  ORG_SETTINGS = '/organization/settings',

  PROJECTS = '/projects',
  PROJECT_DETAILS = '/projects/:projectId',
  PROJECT_SETTINGS = '/projects/:projectId/settings',
  PROJECT_EXPERIMENTS = '/projects/:projectId/experiments',
  PROJECT_WORKSPACES = '/projects/:projectId/workspaces',
}
