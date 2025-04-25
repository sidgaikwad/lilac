import React, { useState } from 'react'; // Removed useEffect, useCallback
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle, CardFooter } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { PlusIcon, PencilIcon, Trash2Icon, CheckIcon, XIcon, Loader2Icon } from 'lucide-react';
import { useNavigate, useParams } from 'react-router-dom'; // Added useParams
import DestructiveActionDialog from '@/components/common/DestructiveActionDialog';
import { toast } from 'sonner';
import useOrganizationStore from '@/store/organizationStore';
import { DataSetStorageEntry } from '@/lib/localStorageUtils'; // Keep type for now
// Import TanStack Query hooks
import { useListDatasets } from '@/services/controlplane-api/useListDatasets.hook';
import {
    useCreateDataset,
    useRenameDataset,
    useDeleteDataset
} from '@/services/controlplane-api/useDatasetMutations.hook';
import { cn } from '@/lib/utils'; // Import cn

// Removed service imports

const DataSetsPage: React.FC = () => {
  // Get projectId from URL params
  const { projectId } = useParams<{ projectId: string }>();
  // Get orgId from store (needed for creating new datasets)
  const { selectedOrganization } = useOrganizationStore();
  const orgId = selectedOrganization?.id;

  const navigate = useNavigate();

  // State for UI interactions (rename/delete modals)
  const [dataSetToDelete, setDataSetToDelete] = useState<DataSetStorageEntry | null>(null);
  const [isDeleteDialogOpen, setIsDeleteDialogOpen] = useState(false);
  const [renamingDataSetId, setRenamingDataSetId] = useState<string | null>(null);
  const [currentNameValue, setCurrentNameValue] = useState("");

  // --- TanStack Query Hooks ---

  // Fetching datasets
  const {
      data: dataSets = [],
      isLoading: isLoadingDatasets,
      isFetching: isFetchingDatasets,
      error: datasetError
  } = useListDatasets({ projectId });

  // Mutations
  const createDatasetMutation = useCreateDataset();
  const renameDatasetMutation = useRenameDataset();
  const deleteDatasetMutation = useDeleteDataset();

  // Combined loading state for disabling actions
  const isMutating = createDatasetMutation.isPending || renameDatasetMutation.isPending || deleteDatasetMutation.isPending;
  const isBusy = isLoadingDatasets || isFetchingDatasets || isMutating; // General busy state

  // --- Action Handlers ---
  const handleImportLocal = () => {
    if (!orgId || !projectId) {
        toast.error("Cannot import: Organization or Project context is missing.");
        return;
    }
    // TODO: Implement actual local file import logic
    console.log('Importing from local...');

    const newDataSetPayload = {
        name: `Local Upload ${new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}`,
        orgId: orgId,
        projectId: projectId,
        source: 'Local Upload',
        originalDataPreview: 'Mock preview data...'
    };
    // Call the mutation hook
    createDatasetMutation.mutate(newDataSetPayload);
  };

  const handleImportS3 = () => {
    if (!projectId) {
        toast.error("Cannot import: Project context is missing.");
        return;
    }
    // TODO: Implement S3 import logic
    console.log('Importing from S3...');
    toast.info('Mock: S3 import initiated.');
  };

  const handleDataSetClick = (datasetId: string) => {
    if (!projectId) return;
    navigate(`/projects/${projectId}/datasets/${datasetId}`);
  };

  // --- Delete Logic ---
  const openDeleteDialog = (dataSet: DataSetStorageEntry) => {
    setDataSetToDelete(dataSet);
    setIsDeleteDialogOpen(true);
  };
  const closeDeleteDialog = () => {
    if (deleteDatasetMutation.isPending) return; // Prevent closing while deleting
    setIsDeleteDialogOpen(false);
    setDataSetToDelete(null);
  };
  const confirmDelete = () => {
    if (dataSetToDelete && projectId) {
        // Pass projectId in context for invalidation
        deleteDatasetMutation.mutate(dataSetToDelete.id, {
            onSuccess: () => {
                closeDeleteDialog(); // Close dialog only on success
            },
            // No context needed here anymore
        });
    }
  };

  // --- Rename Logic ---
   const startRename = (dataSet: DataSetStorageEntry) => {
    setRenamingDataSetId(dataSet.id);
    setCurrentNameValue(dataSet.name);
  };
  const cancelRename = () => {
    setRenamingDataSetId(null);
    setCurrentNameValue("");
  };
  const saveRename = () => {
    if (renamingDataSetId && currentNameValue.trim() && projectId) {
        renameDatasetMutation.mutate({
            datasetId: renamingDataSetId,
            newName: currentNameValue.trim()
        }, {
            onSuccess: () => {
                cancelRename(); // Close input on success
            },
            onError: () => {
                // Optionally keep input open on error? Or reset?
                // For now, it closes via the finally block in the hook if needed
            },
            // No context needed here anymore // Pass context for invalidation
        });
    } else {
        cancelRename(); // Cancel if no ID or name
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
                  Could not determine the current project. Please select one in the header.
              </div>
          );
      }
      // Initial loading state
      if (isLoadingDatasets && dataSets.length === 0) {
          return (
              <div className="text-center py-10 flex items-center justify-center text-muted-foreground">
                  <Loader2Icon className="mr-2 h-5 w-5 animate-spin" /> Loading datasets...
              </div>
          );
      }
      // Error state (only if no data is available)
      if (datasetError && dataSets.length === 0) {
           return (
              <div className="text-center py-10 border rounded-lg bg-destructive/10 text-destructive">
                  Error loading datasets: {datasetError.message}
              </div>
          );
      }
      // Empty state (when not loading/fetching and data is empty)
      if (dataSets.length === 0 && !isFetchingDatasets) {
          return (
             <div className="text-center text-muted-foreground col-span-full mt-8">
               No data sets found for this project. Import your first data set to get started.
             </div>
          );
      }

      // Grid view
      return (
          <div className={cn(
              "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
              isFetchingDatasets && !isLoadingDatasets && "opacity-60 transition-opacity duration-300" // Dim on refetch
          )}>
              {dataSets.map((dataSet) => (
                <Card
                  key={dataSet.id}
                  className="hover:shadow-lg transition-shadow"
                >
                  <CardHeader className="flex-grow pb-2">
                     {renamingDataSetId === dataSet.id ? (
                        <div className="flex gap-1 items-center">
                          <Input value={currentNameValue} onChange={handleRenameInputChange} onKeyDown={handleRenameInputKeyDown} onBlur={saveRename} className="text-lg h-8 flex-grow" autoFocus disabled={renameDatasetMutation.isPending} />
                          <Button variant="ghost" size="icon" className="h-8 w-8 hover:bg-accent" onClick={saveRename} title="Save" disabled={renameDatasetMutation.isPending}>
                              {renameDatasetMutation.isPending ? <Loader2Icon className="h-4 w-4 animate-spin" /> : <CheckIcon className="h-4 w-4"/>}
                          </Button>
                          <Button variant="ghost" size="icon" className="h-8 w-8 hover:bg-accent" onClick={cancelRename} title="Cancel" disabled={renameDatasetMutation.isPending}><XIcon className="h-4 w-4"/></Button>
                        </div>
                      ) : (
                         <CardTitle
                           className="text-lg cursor-pointer hover:underline"
                           onClick={() => handleDataSetClick(dataSet.id)}
                         >
                           {dataSet.name}
                         </CardTitle>
                      )}
                  </CardHeader>
                  <CardContent className="pb-4">
                    <p className="text-sm text-muted-foreground">Source: {dataSet.source}</p>
                    <p className="text-sm text-muted-foreground">Created: {new Date(dataSet.createdAt).toLocaleDateString()}</p>
                  </CardContent>
                   <CardFooter className="border-t pt-3 pb-3 flex justify-end gap-1">
                      <Button variant="ghost" size="icon" className="h-7 w-7" title="Rename Dataset" onClick={() => startRename(dataSet)} disabled={renamingDataSetId === dataSet.id || isMutating || isFetchingDatasets}>
                        <PencilIcon className="h-4 w-4" />
                      </Button>
                      <Button variant="ghost" size="icon" className="h-7 w-7 text-destructive hover:bg-destructive/10 hover:text-destructive" title="Delete Dataset" onClick={() => openDeleteDialog(dataSet)} disabled={isMutating || isFetchingDatasets}>
                        <Trash2Icon className="h-4 w-4" />
                      </Button>
                    </CardFooter>
                </Card>
              ))}
            </div>
      );
  };

  return (
    <div className="container mx-auto p-4 md:p-6 lg:p-8">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-semibold">Data Sets</h1>
         {/* Optional: Show spinner during refetch */}
         {isFetchingDatasets && !isLoadingDatasets && <Loader2Icon className="h-5 w-5 animate-spin text-muted-foreground ml-2" />}
        <div className="space-x-2 ml-auto">
          <Button onClick={handleImportLocal} variant="outline" disabled={isBusy || !projectId || !orgId}>
            {createDatasetMutation.isPending ? <Loader2Icon className="mr-2 h-4 w-4 animate-spin" /> : <PlusIcon className="mr-2 h-4 w-4" />}
             Import from Local
          </Button>
          <Button onClick={handleImportS3} variant="outline" disabled> {/* Keep S3 disabled */}
            <PlusIcon className="mr-2 h-4 w-4" /> Import from S3
          </Button>
        </div>
      </div>

      {renderContent()}

      {/* Delete Confirmation Dialog */}
      {dataSetToDelete && (
          <DestructiveActionDialog
              isOpen={isDeleteDialogOpen}
              onClose={closeDeleteDialog}
              onConfirm={confirmDelete}
              title={`Delete Dataset: ${dataSetToDelete.name}`}
              description={<>Are you sure you want to permanently delete this dataset?<strong className="block mt-2 text-destructive">This action cannot be undone.</strong></>}
              confirmationText="DELETE"
              confirmButtonText="Delete Dataset"
              isConfirming={deleteDatasetMutation.isPending} // Use mutation pending state
          />
      )}
    </div>
  );
};

export default DataSetsPage;