// Placeholder types related to pipelines, steps, parameters, etc.
// These should align closely with backend models (e.g., PipelineStageConfig)

export interface ParameterDefinition {
  name: string;
  type: 'string' | 'number' | 'boolean' | 'enum' | 's3_path'; // Example types
  label: string;
  required: boolean;
  defaultValue?: any;
  options?: string[]; // For enum type
  description?: string;
}

export interface StepDefinition {
  id: string; // Unique identifier for the step *type* (maps to backend's step_definition_id)
  type: string; // e.g., "BlurDetector", "ResolutionStandardizer"
  label: string;
  description?: string;
  category?: string; // e.g., "Input", "Processing", "Output"
  parameters: ParameterDefinition[];
  // Input/Output handle definitions might go here if needed by frontend
}

export interface PipelineNodeData {
  label: string;
  type: string; // Corresponds to StepDefinition.type
  parameters: Record<string, any>; // User-configured parameters
  // Add any other node-specific data needed by the frontend
}

// Simplified representation for API (align with backend)
export interface PipelineStepConfig {
  id: string; // Unique ID for this step *instance* within the pipeline
  step_type: string; // Matches StepDefinition.type
  parameters: Record<string, any>;
  // Position/layout info might be stored separately or here
  position?: { x: number; y: number };
}

export interface PipelineConnectionConfig {
  from_step_id: string;
  to_step_id: string;
  // Potentially handle IDs if multiple inputs/outputs exist
  // from_handle?: string;
  // to_handle?: string;
}

export interface PipelineDefinition {
  id: string;
  name: string;
  organization_id: string;
  // Representation of the pipeline structure
  steps: PipelineStepConfig[];
  connections: PipelineConnectionConfig[];
  // Add versioning info
  version_id?: string;
  created_at?: string;
  updated_at?: string;
}

// Type for the list view on the dashboard
export interface PipelineListItem {
  id: string;
  name: string;
  last_modified: string; // Or Date object
  // status?: string; // If available
}