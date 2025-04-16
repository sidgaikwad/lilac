import React, { useState, useEffect, useCallback } from 'react';
import { useNavigate, Link } from 'react-router-dom';
import { Button } from '@/components/ui/button';
import { Card, CardHeader, CardTitle, CardDescription, CardFooter } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { v4 as uuidv4 } from 'uuid';
import { listStoredPipelines, savePipelineEntry, renamePipeline, deletePipelineEntry, PipelineStorageEntry } from '@/lib/localStorageUtils';
import { PencilIcon, Trash2Icon, CheckIcon, XIcon } from 'lucide-react';
import DestructiveActionDialog from '@/components/common/DestructiveActionDialog';
import { toast } from 'sonner';
import { cn } from '@/lib/utils';

interface DashboardPipelineItem {
  id: string;
  name: string;
  lastModified: string;
}

const buttonFocusStyle = "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 dark:focus-visible:ring-offset-gray-950";

const DashboardPage: React.FC = () => {
  const navigate = useNavigate();
  const [pipelines, setPipelines] = useState<DashboardPipelineItem[]>([]);
  const [pipelineToDelete, setPipelineToDelete] = useState<DashboardPipelineItem | null>(null);
  const [isDeleteDialogOpen, setIsDeleteDialogOpen] = useState(false);
  const [renamingPipelineId, setRenamingPipelineId] = useState<string | null>(null);
  const [currentNameValue, setCurrentNameValue] = useState("");

  const loadPipelines = useCallback(() => {
    setPipelines(listStoredPipelines());
  }, []);

  useEffect(() => {
    loadPipelines();
  }, [loadPipelines]);

  const handleCreatePipeline = () => {
    const newPipelineId = uuidv4();
    const newPipelineName = `New Pipeline ${new Date().toLocaleTimeString()}`;
    const newEntry: PipelineStorageEntry = { id: newPipelineId, name: newPipelineName, versions: [] };
    savePipelineEntry(newEntry);
    loadPipelines();
    navigate(`/pipelines/${newPipelineId}`);
  };

  const openDeleteDialog = (pipeline: DashboardPipelineItem) => {
    setPipelineToDelete(pipeline);
    setIsDeleteDialogOpen(true);
  };

  const closeDeleteDialog = () => {
    setIsDeleteDialogOpen(false);
    setPipelineToDelete(null);
  };

  const confirmDelete = () => {
    if (pipelineToDelete) {
      const success = deletePipelineEntry(pipelineToDelete.id);
      if (success) {
        toast.success(`Pipeline "${pipelineToDelete.name}" deleted.`);
        loadPipelines();
      } else {
        toast.error(`Failed to delete pipeline "${pipelineToDelete.name}".`);
      }
    }
  };

  const startRename = (pipeline: DashboardPipelineItem) => {
    setRenamingPipelineId(pipeline.id);
    setCurrentNameValue(pipeline.name);
  };

  const cancelRename = () => {
    setRenamingPipelineId(null);
    setCurrentNameValue("");
  };

  const saveRename = () => {
    if (renamingPipelineId && currentNameValue.trim()) {
      const success = renamePipeline(renamingPipelineId, currentNameValue.trim());
      if (success) {
        toast.success("Pipeline renamed.");
        loadPipelines();
      } else {
        toast.error("Failed to rename pipeline.");
      }
    }
    cancelRename();
  };

  const handleRenameInputChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setCurrentNameValue(event.target.value);
  };

   const handleRenameInputKeyDown = (event: React.KeyboardEvent<HTMLInputElement>) => {
    if (event.key === 'Enter') {
      saveRename();
    } else if (event.key === 'Escape') {
      cancelRename();
    }
  };


  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <h1 className="text-3xl font-bold">Pipelines</h1>
        <Button
          onClick={handleCreatePipeline}
          className={cn(buttonFocusStyle)} // Use default variant + focus
        >
          Create New Pipeline
        </Button>
      </div>

      {pipelines.length === 0 ? (
        // Use theme card/muted colors for empty state
        <div className="text-center py-10 border rounded-lg bg-card">
          <h3 className="text-xl font-semibold mb-2">No Pipelines Yet</h3>
          <p className="text-muted-foreground mb-4">Get started by creating your first pipeline.</p>
          <Button
            onClick={handleCreatePipeline}
             className={cn(buttonFocusStyle)} // Use default variant + focus
          >
            Create Pipeline
          </Button>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {pipelines.map(p => (
            // Card uses theme variables internally
            <Card key={p.id} className="flex flex-col">
              <CardHeader className="flex-grow">
                {renamingPipelineId === p.id ? (
                  <div className="flex gap-1 items-center">
                    <Input
                      value={currentNameValue}
                      onChange={handleRenameInputChange}
                      onKeyDown={handleRenameInputKeyDown}
                      onBlur={saveRename}
                      className="text-lg h-8 flex-grow" // Input uses theme variables
                      autoFocus
                    />
                    {/* Buttons use theme variables via variants */}
                    <Button variant="ghost" size="icon" className={cn("h-8 w-8 text-green-600 hover:bg-green-100", buttonFocusStyle)} onClick={saveRename}><CheckIcon className="h-4 w-4"/></Button>
                    <Button variant="ghost" size="icon" className={cn("h-8 w-8 text-red-600 hover:bg-red-100", buttonFocusStyle)} onClick={cancelRename}><XIcon className="h-4 w-4"/></Button>
                  </div>
                ) : (
                  // Use theme primary color for link
                  <Link to={`/pipelines/${p.id}`} className="text-primary hover:underline">
                    <CardTitle className="text-lg">{p.name}</CardTitle>
                  </Link>
                )}
                {/* CardDescription uses theme muted foreground */}
                <CardDescription>
                  Last Modified: {new Date(p.lastModified).toLocaleString()}
                </CardDescription>
              </CardHeader>
              {/* CardFooter uses theme border */}
              <CardFooter className="border-t pt-4 flex justify-end gap-2">
                <Button variant="ghost" size="icon" title="Rename" onClick={() => startRename(p)} disabled={renamingPipelineId === p.id} className={cn(buttonFocusStyle)}>
                  <PencilIcon className="h-4 w-4" />
                </Button>
                <Button
                  variant="ghost"
                  size="icon"
                  // Use theme destructive text color and hover
                  className={cn("text-destructive hover:bg-destructive/10 hover:text-destructive", buttonFocusStyle)}
                  title="Delete Pipeline"
                  onClick={() => openDeleteDialog(p)}
                >
                  <Trash2Icon className="h-4 w-4" />
                </Button>
              </CardFooter>
            </Card>
          ))}
        </div>
      )}

      {pipelineToDelete && (
        <DestructiveActionDialog
          isOpen={isDeleteDialogOpen}
          onClose={closeDeleteDialog}
          onConfirm={confirmDelete}
          title={`Delete Pipeline: ${pipelineToDelete.name}`}
          description={
            <>
              Are you sure you want to permanently delete this pipeline and all its versions?
              <strong className="block mt-2 text-destructive">This action cannot be undone.</strong>
            </>
          }
          confirmationText="DELETE"
          confirmButtonText="Delete Pipeline"
        />
      )}
    </div>
  );
};

export default DashboardPage;