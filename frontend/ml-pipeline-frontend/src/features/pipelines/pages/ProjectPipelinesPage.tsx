import React, { useState } from 'react';
import { useNavigate, Link, useParams } from 'react-router-dom';
import { Button } from '@/components/ui/button';
import { Card, CardHeader, CardTitle, CardDescription, CardFooter } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
// Removed localStorageUtils imports
import { PencilIcon, Trash2Icon, CheckIcon, XIcon, Loader2Icon, WorkflowIcon } from 'lucide-react';
import DestructiveActionDialog from '@/components/common/DestructiveActionDialog';
import { toast } from 'sonner';
import { cn } from '@/lib/utils';
import { useQuery, useQueryClient } from '@tanstack/react-query'; // Import useQueryClient
import { PipelineListItem } from '@/types';
// Import TanStack Query hooks for pipelines
import { useListPipelines } from '@/services/controlplane-api/useListPipelines.hook';
import {
    useCreatePipeline,
    useRenamePipeline,
    useDeletePipeline
} from '@/services/controlplane-api/usePipelineMutations.hook';

// Removed local mock fetchPipelines function

const buttonFocusStyle = "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 dark:focus-visible:ring-offset-gray-950";

const ProjectPipelinesPage: React.FC = () => {
  const navigate = useNavigate();
  const { projectId } = useParams<{ projectId: string }>(); // Get projectId from URL

  // State for UI interactions
  const [pipelineToDelete, setPipelineToDelete] = useState<PipelineListItem | null>(null);
  const [isDeleteDialogOpen, setIsDeleteDialogOpen] = useState(false);
  const [renamingPipelineId, setRenamingPipelineId] = useState<string | null>(null);
  const [currentNameValue, setCurrentNameValue] = useState("");

  // --- TanStack Query Hooks ---
  const {
      data: pipelines = [],
      isLoading: isLoadingPipelines,
      isFetching: isFetchingPipelines,
      error: pipelineError
  } = useListPipelines({ projectId }); // Use the hook

  const createPipelineMutation = useCreatePipeline();
  const renamePipelineMutation = useRenamePipeline();
  const deletePipelineMutation = useDeletePipeline();

  // Combined loading state
  const isMutating = createPipelineMutation.isPending || renamePipelineMutation.isPending || deletePipelineMutation.isPending;
  const isBusy = isLoadingPipelines || isFetchingPipelines || isMutating;

  // --- Action Handlers ---
  const handleCreatePipeline = () => {
    if (!projectId) {
        toast.error("Cannot create pipeline: Project ID is missing from URL.");
        return;
    }
    const newPipelineName = `New Pipeline ${new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}`;
    createPipelineMutation.mutate({
        name: newPipelineName,
        projectId: projectId,
    }, {
        onSuccess: (data) => {
            // Navigate on successful creation
            navigate(`/projects/${projectId}/pipelines/${data.id}`);
        }
    });
  };

  // --- Delete Logic ---
  const openDeleteDialog = (pipeline: PipelineListItem) => {
    setPipelineToDelete(pipeline);
    setIsDeleteDialogOpen(true);
  };
  const closeDeleteDialog = () => {
     if (deletePipelineMutation.isPending) return;
    setIsDeleteDialogOpen(false);
    setPipelineToDelete(null);
  };
  const confirmDelete = () => {
    if (pipelineToDelete && projectId) {
        deletePipelineMutation.mutate(pipelineToDelete.id, {
            onSuccess: () => {
                closeDeleteDialog();
            },
            // context: { projectId } // Not needed anymore
        });
    }
  };

  // --- Rename Logic ---
  const startRename = (pipeline: PipelineListItem) => {
    setRenamingPipelineId(pipeline.id);
    setCurrentNameValue(pipeline.name);
  };
  const cancelRename = () => {
    setRenamingPipelineId(null);
    setCurrentNameValue("");
  };
  const saveRename = () => {
    if (renamingPipelineId && currentNameValue.trim() && projectId) {
        renamePipelineMutation.mutate({
            pipelineId: renamingPipelineId,
            newName: currentNameValue.trim()
        }, {
            onSuccess: () => {
                cancelRename();
            },
            // context: { projectId } // Not needed anymore
        });
    } else {
        cancelRename();
    }
  };
  const handleRenameInputChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setCurrentNameValue(event.target.value);
  };
   const handleRenameInputKeyDown = (event: React.KeyboardEvent<HTMLInputElement>) => {
    if (event.key === 'Enter') saveRename();
    else if (event.key === 'Escape') cancelRename();
  };

   // --- Render Logic ---
  const renderContent = () => {
      if (!projectId) {
           return (
              <div className="text-center py-10 border rounded-lg bg-card text-muted-foreground">
                  Project ID not found in URL.
              </div>
          );
      }
      if (isLoadingPipelines && pipelines.length === 0) {
          return (
              <div className="text-center py-10 flex items-center justify-center text-muted-foreground">
                  <Loader2Icon className="mr-2 h-5 w-5 animate-spin" /> Loading pipelines...
              </div>
          );
      }
      if (pipelineError && pipelines.length === 0) {
          return (
              <div className="text-center py-10 border rounded-lg bg-destructive/10 text-destructive">
                  Error loading pipelines: {pipelineError.message}
              </div>
          );
      }
      if (pipelines.length === 0 && !isFetchingPipelines) {
          return (
              <div className="text-center py-10 border rounded-lg bg-card">
                  <h3 className="text-xl font-semibold mb-2">No Pipelines Yet</h3>
                  <p className="text-muted-foreground mb-4">Get started by creating your first pipeline for this project.</p>
                  <Button onClick={handleCreatePipeline} className={cn(buttonFocusStyle)} disabled={createPipelineMutation.isPending}>
                      {createPipelineMutation.isPending && <Loader2Icon className="mr-2 h-4 w-4 animate-spin" />}
                      Create Pipeline
                  </Button>
              </div>
          );
      }

      return (
          <div className={cn(
              "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
              isFetchingPipelines && !isLoadingPipelines && "opacity-60 transition-opacity duration-300"
          )}>
              {pipelines.map(p => (
                  <Card key={p.id} className="flex flex-col">
                      <CardHeader className="flex-grow">
                          {renamingPipelineId === p.id ? (
                              <div className="flex gap-1 items-center">
                                  <Input value={currentNameValue} onChange={handleRenameInputChange} onKeyDown={handleRenameInputKeyDown} onBlur={saveRename} className="text-lg h-8 flex-grow" autoFocus disabled={renamePipelineMutation.isPending} />
                                  <Button variant="ghost" size="icon" className={cn("h-8 w-8 text-green-600 hover:bg-green-100", buttonFocusStyle)} onClick={saveRename} disabled={renamePipelineMutation.isPending}>
                                      {renamePipelineMutation.isPending ? <Loader2Icon className="h-4 w-4 animate-spin" /> : <CheckIcon className="h-4 w-4"/>}
                                  </Button>
                                  <Button variant="ghost" size="icon" className={cn("h-8 w-8 text-red-600 hover:bg-red-100", buttonFocusStyle)} onClick={cancelRename} disabled={renamePipelineMutation.isPending}><XIcon className="h-4 w-4"/></Button>
                              </div>
                          ) : (
                              <Link to={`/projects/${projectId}/pipelines/${p.id}`} className="text-primary hover:underline group">
                                  <CardTitle className="text-lg flex items-center">
                                      <WorkflowIcon className="mr-2 h-5 w-5 text-muted-foreground group-hover:text-primary transition-colors" />
                                      {p.name}
                                  </CardTitle>
                              </Link>
                          )}
                          <CardDescription>
                              Last Modified: {p.lastModified === new Date(0).toISOString() ? 'N/A' : new Date(p.lastModified).toLocaleString()}
                          </CardDescription>
                      </CardHeader>
                      <CardFooter className="border-t pt-4 flex justify-end gap-2">
                          <Button variant="ghost" size="icon" title="Rename Pipeline" onClick={() => startRename(p)} disabled={renamingPipelineId === p.id || isMutating || isFetchingPipelines} className={cn(buttonFocusStyle)}>
                              <PencilIcon className="h-4 w-4" />
                          </Button>
                          <Button variant="ghost" size="icon" className={cn("text-destructive hover:bg-destructive/10 hover:text-destructive", buttonFocusStyle)} title="Delete Pipeline" onClick={() => openDeleteDialog(p)} disabled={isMutating || isFetchingPipelines}>
                              <Trash2Icon className="h-4 w-4" />
                          </Button>
                      </CardFooter>
                  </Card>
              ))}
          </div>
      );
  };

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <h1 className="text-3xl font-bold">Pipelines</h1>
        {isFetchingPipelines && !isLoadingPipelines && <Loader2Icon className="h-5 w-5 animate-spin text-muted-foreground ml-2" />}
        <Button onClick={handleCreatePipeline} className={cn(buttonFocusStyle, "ml-auto")} disabled={!projectId || createPipelineMutation.isPending || isFetchingPipelines}>
           {createPipelineMutation.isPending && <Loader2Icon className="mr-2 h-4 w-4 animate-spin" />}
           Create New Pipeline
        </Button>
      </div>

      {renderContent()}

      {pipelineToDelete && (
        <DestructiveActionDialog
            isOpen={isDeleteDialogOpen}
            onClose={closeDeleteDialog}
            onConfirm={confirmDelete}
            title={`Delete Pipeline: ${pipelineToDelete.name}`}
            description={<>Are you sure you want to permanently delete this pipeline and all its versions?<strong className="block mt-2 text-destructive">This action cannot be undone.</strong></>}
            confirmationText="DELETE"
            confirmButtonText="Delete Pipeline"
            isConfirming={deletePipelineMutation.isPending} // Use mutation pending state
        />
      )}
    </div>
  );
};

export default ProjectPipelinesPage;