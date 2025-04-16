ALTER TABLE step_definitions DROP COLUMN parameter_definitions;
ALTER TABLE step_definitions ADD schema jsonb NOT NULL DEFAULT '{}'::jsonb;