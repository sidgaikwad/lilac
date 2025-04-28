import { PipelineSummary, Step, StepDefinition } from '@/types';

export interface LoginResponse {
  accessToken: string;
  tokenType: 'Bearer';
}

export interface RegisterUserRequest {
  email: string;
  password: string;
}

export interface RegisterUserResponse {
  id: string;
}

export interface GetOrganizationResponse {
  id: string;
  name: string;
}

export interface ListOrganizationsResponse {
  organizations: GetOrganizationResponse[];
}

export interface CreateOrganizationRequest {
  name: string;
}

export interface CreateOrganizationResponse {
  id: string;
}

export interface GetProjectResponse {
  id: string;
  name: string;
  organizationId: string;
}

export interface ListProjectsResponse {
  projects: GetProjectResponse[];
}

export interface CreateProjectRequest {
  name: string;
  organizationId: string;
}

export interface CreateProjectResponse {
  id: string;
}

export interface ListPipelinesResponse {
  pipelines: PipelineSummary[];
}

export interface CreatePipelineRequest {
  name: string;
  projectId: string;
}

export interface CreatePipelineResponse {
  id: string;
}
export interface UpdatePipelineRequest {
  pipelineId: string;
  name?: string;
  description?: string;
  steps?: Pick<Step, 'stepId' | 'stepDefinitionId' | 'stepParameters'>[];
  stepConnections?: [string, string][];
}

export interface RunPipelineResponse {
  jobId: string;
}

export interface ListStepDefinitionsResponse {
  stepDefinitions: StepDefinition[];
}
