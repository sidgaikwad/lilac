CREATE TABLE steps (
    step_id uuid PRIMARY KEY,
    step_type text NOT NULL,
    parameter_definitions jsonb[] NOT NULL DEFAULT array[]::jsonb[]
);
CREATE TABLE step_instances (
    step_instance_id uuid PRIMARY KEY,
    pipeline_id uuid NOT NULL REFERENCES pipelines(pipeline_id),
    step_id uuid NOT NULL REFERENCES steps(step_id),
    previous_step uuid REFERENCES step_instances(step_instance_id),
    next_step uuid REFERENCES step_instances(step_instance_id),
    step_parameters jsonb NOT NULL DEFAULT '{}'::jsonb
);
-- register the first step type
INSERT INTO steps(step_id, step_type, parameter_definitions)
    VALUES ('4607fc3c-a6ed-4e72-9849-d5b6713cc346', 'NoOp', array[]::jsonb[]);