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
  stepDefinition: StepDefinition; // Store the definition for easy access
}

// Simplified representation for API (align with backend)
export interface PipelineStepConfig {
  id: string; // Unique ID for this step *instance* within the pipeline
  step_type: string; // Matches StepDefinition.type
  parameters: Record<string, any>;
  position?: { x: number; y: number };
}

export interface PipelineConnectionConfig {
  from_step_id: string;
  to_step_id: string;
  // from_handle?: string;
  // to_handle?: string;
}

export interface PipelineDefinition {
  id: string;
  name: string;
  organization_id: string;
  steps: PipelineStepConfig[];
  connections: PipelineConnectionConfig[];
  version_id?: string;
  created_at?: string;
  updated_at?: string;
}

// Type for the list view on the dashboard
export interface PipelineListItem {
  id: string;
  name: string;
  lastModified: string; // Use camelCase to match localStorageUtils return type
}