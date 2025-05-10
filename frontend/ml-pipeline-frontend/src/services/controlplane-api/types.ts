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

export interface RunPipelinePayload {
  pipelineId: string;
  datasetPath: string;
}
export interface RunPipelineResponse {
  jobId: string;
}

export interface ListStepDefinitionsResponse {
  stepDefinitions: StepDefinition[];
}

export interface ListDatasetsResponse {
  datasets: string[];
}

export interface JobOutputSummary {
  jobId: string;
  inputDatasetName: string | null; // Can be null if dataset_path was null for the job
  completedAt: string | null; // Assuming string representation of timestamp
}

export interface JobOutputImages {
  jobId: string;
  images: string[];
}

export interface CreateDatasetRequest {
  datasetName: string;
  description?: string;
  images: string[];
  projectId: string;
}

export interface CreateDatasetResponse {
  id: string;
}