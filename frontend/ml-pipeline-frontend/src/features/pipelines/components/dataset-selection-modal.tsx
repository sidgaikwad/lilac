import React, { useState } from 'react';
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
  DialogClose,
} from '@/components/ui/dialog';
import { Button } from '@/components/ui/button';
import { ScrollArea } from '@/components/ui/scroll-area';
import { useListDatasets } from '@/services';
import { Skeleton } from '@/components/ui/skeleton';
import { useParams } from 'react-router-dom';

interface DatasetSelectionModalProps {
  isOpen: boolean;
  onClose: () => void;
  onSelectDataset: (datasetId: string) => void;
}

const DatasetSelectionModal: React.FC<DatasetSelectionModalProps> = ({
  isOpen,
  onClose,
  onSelectDataset,
}) => {
  const [selectedDataset, setSelectedDataset] = useState<string | null>(null);
  const { projectId } = useParams();
  const {
    data: datasets,
    isLoading,
    isError,
    error,
  } = useListDatasets({ projectId });

  const handleDatasetSelect = (datasetId: string) => {
    setSelectedDataset(datasetId);
  };

  const handleConfirm = () => {
    if (selectedDataset) {
      onSelectDataset(selectedDataset);
      onClose();
    }
  };

  return (
    <Dialog open={isOpen} onOpenChange={onClose}>
      <DialogContent className="sm:max-w-[425px]">
        <DialogHeader>
          <DialogTitle>Select Dataset</DialogTitle>
        </DialogHeader>
        <div className="py-4">
          {isLoading && (
            <div className="space-y-2">
              <Skeleton className="h-8 w-full" />
              <Skeleton className="h-8 w-full" />
              <Skeleton className="h-8 w-full" />
            </div>
          )}
          {isError && (
            <p className="text-sm text-red-500">
              Error fetching datasets:{' '}
              {error?.error || 'Could not load datasets.'}
            </p>
          )}
          {!isLoading && !isError && datasets && (
            <ScrollArea className="h-[200px] w-full rounded-md border p-4">
              {datasets.length === 0 && (
                <p className="text-muted-foreground text-center text-sm">
                  No datasets found.
                </p>
              )}
              {datasets.map((dataset) => (
                <Button
                  key={dataset.id}
                  variant={
                    selectedDataset === dataset.name ? 'default' : 'outline'
                  }
                  className="mb-2 w-full justify-start"
                  onClick={() => handleDatasetSelect(dataset.id)}
                >
                  {dataset.name}
                </Button>
              ))}
            </ScrollArea>
          )}
        </div>
        <DialogFooter>
          <DialogClose asChild>
            <Button variant="outline" onClick={onClose}>
              Cancel
            </Button>
          </DialogClose>
          <Button
            onClick={handleConfirm}
            disabled={!selectedDataset || isLoading}
          >
            Run with Selected Dataset
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};

export default DatasetSelectionModal;
