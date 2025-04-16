import React from 'react';
import { Button } from '@/components/ui/button'; // Assuming we'll add buttons here

interface PipelineEditorTopBarProps {
  pipelineName?: string;
  onSave?: () => void; // Placeholder for save action
  // Add props for version selection later
}

const PipelineEditorTopBar: React.FC<PipelineEditorTopBarProps> = ({
  pipelineName,
  onSave,
}) => {
  return (
    <header className="h-16 bg-gray-50 dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 flex items-center justify-between px-4 shrink-0">
      <h1 className="text-xl font-semibold truncate" title={pipelineName}>
        {pipelineName || 'Pipeline Editor'}
      </h1>
      <div className="flex items-center gap-2">
        {/* Placeholder for Version Selector */}
        <span className="text-sm text-muted-foreground">Version: Latest</span>
        <Button onClick={onSave} size="sm" disabled={!onSave}>
          Save
        </Button>
      </div>
    </header>
  );
};

export default PipelineEditorTopBar;