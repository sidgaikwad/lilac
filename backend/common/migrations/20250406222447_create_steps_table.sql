CREATE TABLE step_definitions (
    step_definition_id uuid PRIMARY KEY,
    step_type text NOT NULL,
    parameter_definitions jsonb[] NOT NULL DEFAULT array[]::jsonb[]
);

CREATE TABLE steps (
    step_id uuid PRIMARY KEY,
    pipeline_id uuid NOT NULL REFERENCES pipelines(pipeline_id),
    step_definition_id uuid NOT NULL REFERENCES step_definitions(step_definition_id),
    step_parameters jsonb NOT NULL DEFAULT '{}'::jsonb
);
CREATE INDEX idx_steps_pipeline_id ON steps (pipeline_id);

CREATE TABLE step_connections (
    pipeline_id uuid NOT NULL REFERENCES pipelines(pipeline_id),
    from_step_id uuid NOT NULL REFERENCES steps(step_id),
    to_step_id uuid NOT NULL REFERENCES steps(step_id),
    PRIMARY KEY (from_step_id, to_step_id)
);
CREATE INDEX idx_step_connections_pipeline_id ON steps (pipeline_id);