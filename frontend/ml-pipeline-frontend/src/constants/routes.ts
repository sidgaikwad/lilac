export enum Routes {
  LOGIN = '/login',
  LOGOUT = '/logout',
  SIGNUP = '/signup',
  HOME = '/home',
  ORGANIZATIONS = '/organizations',
  PROJECTS = '/organizations/:organizationId/projects',
  PROJECT_DETAILS = '/projects/:projectId',
  PROJECT_PIPELINES = '/projects/:projectId/pipelines',
  PROJECT_PIPELINE_DETAILS = '/projects/:projectId/pipelines/:pipelineId',
  PROJECT_DATASETS = '/projects/:projectId/datasets',
  PROJECT_DATASET_DETAILS = '/projects/:projectId/datasets/:datasetId',
}
