export interface Step {
  stepId: string;
  stepDefinitionId: string;
  stepType: string;
  stepParameters: Record<string, string | number | boolean | object>;
}

export interface StepDefinition {
  id: string;
  name: string;
  description?: string;
  category: string;
  stepType: string;
  schema: unknown;
  inputs: string[];
  outputs: string[];
}
