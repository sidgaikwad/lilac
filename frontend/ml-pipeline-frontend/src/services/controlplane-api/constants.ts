export const BASE_URL = 'http://localhost:3000';

export enum QueryKeys {
  LOGIN = 'Login',

  GET_USER = 'GetUser',
  CREATE_USER = 'CreateUser',

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

  GET_PIPELINE = 'GetPipeline',
  CREATE_PIPELINE = 'CreatePipeline',
  UPDATE_PIPELINE = 'UpdatePipeline',
  LIST_PIPELINE = 'ListPipelines',
  DELETE_PIPELINE = 'DeletePipeline',
  LIST_DATASETS = 'ListDataSets',

  LIST_STEP_DEFINITIONS = 'ListStepDefinitions',
}
