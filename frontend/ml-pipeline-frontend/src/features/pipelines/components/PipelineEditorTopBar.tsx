import React, { useState, useEffect } from 'react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';

import { toast } from 'sonner';
import { cn } from '@/lib/utils';


interface PipelineVersion {
  versionId: string;
  timestamp: string;
  
}

interface PipelineEditorTopBarProps {
  pipelineId: string | undefined;
  pipelineName?: string;
  onSave?: () => void;
  isSaving?: boolean;
  onRename: (pipelineId: string, newName: string) => boolean;
  versions: PipelineVersion[];
  selectedVersionId: string | undefined;
  onSelectVersion: (versionId: string) => void;
}


const buttonFocusStyle =
  'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 dark:focus-visible:ring-offset-gray-950';

const PipelineEditorTopBar: React.FC<PipelineEditorTopBarProps> = ({
  pipelineId,
  pipelineName = 'Untitled Pipeline',
  onSave,
  isSaving = false,
  onRename,
  versions = [],
  selectedVersionId,
  onSelectVersion,
}) => {
  const [isEditingName, setIsEditingName] = useState(false);
  const [currentName, setCurrentName] = useState(pipelineName);

  useEffect(() => {
    setCurrentName(pipelineName);
  }, [pipelineName]);

  const formatTimestamp = (isoString: string) => {
    return new Date(isoString).toLocaleString();
  };

  const handleNameChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setCurrentName(event.target.value);
  };

  const handleNameBlur = () => {
    setIsEditingName(false);
    if (pipelineId && currentName.trim() && currentName !== pipelineName) {
      const success = onRename(pipelineId, currentName.trim());
      if (success) {
        toast.success('Pipeline renamed successfully.');
      } else {
        toast.error('Failed to rename pipeline.');
        setCurrentName(pipelineName);
      }
    } else {
      setCurrentName(pipelineName);
    }
  };

  const handleNameKeyDown = (event: React.KeyboardEvent<HTMLInputElement>) => {
    if (event.key === 'Enter') {
      handleNameBlur();
    } else if (event.key === 'Escape') {
      setCurrentName(pipelineName);
      setIsEditingName(false);
    }
  };

  return (
    
    <header className="h-16 bg-card border-b border-border flex items-center justify-between px-4 shrink-0">
      {isEditingName ? (
        <Input
          type="text"
          value={currentName}
          onChange={handleNameChange}
          onBlur={handleNameBlur}
          onKeyDown={handleNameKeyDown}
          className="text-xl font-semibold h-9" 
          autoFocus
          maxLength={100}
        />
      ) : (
        <h1
          
          className="text-xl font-semibold truncate cursor-pointer hover:text-primary"
          title={`Click to rename "${pipelineName}"`}
          onClick={() => setIsEditingName(true)}
        >
          {currentName}
        </h1>
      )}

      <div className="flex items-center gap-3">
        {versions.length > 0 ? (
          <Select
            value={selectedVersionId}
            onValueChange={(value) => value && onSelectVersion(value)}
          >
            {}
            <SelectTrigger className="w-[280px] text-sm">
              <SelectValue placeholder="Select version..." />
            </SelectTrigger>
            {}
            <SelectContent>
              {versions.map((v) => (
                <SelectItem key={v.versionId} value={v.versionId}>
                  Version saved at: {formatTimestamp(v.timestamp)}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        ) : (
          
          <span className="text-sm text-muted-foreground">
            No saved versions
          </span>
        )}

        {}
        <Button
          onClick={onSave}
          size="sm"
          disabled={!onSave || isSaving}
          className={cn(buttonFocusStyle)}
        >
          {isSaving ? 'Saving...' : 'Save New Version'}
        </Button>
      </div>
    </header>
  );
};

export default PipelineEditorTopBar;
