export enum Routes {
  HOME = '/',
  LOGIN = '/login',
  LOGOUT = '/logout',
  SIGNUP = '/signup',
  ACCOUNT_SETTINGS = '/account/settings',
  PROJECT_DETAILS = '/projects/:projectId',
  PROJECT_DATASETS = '/projects/:projectId/datasets',
  PROJECT_DATASET_DETAILS = '/projects/:projectId/datasets/:datasetId',
  PROJECT_SETTINGS = '/projects/:projectId/settings',
  PROJECT_EXPERIMENTS = '/projects/:projectId/experiments',
  PROJECT_NOTEBOOKS = '/projects/:projectId/notebooks',
  PROJECT_WORKSPACES = '/projects/:projectId/workspaces',
}
