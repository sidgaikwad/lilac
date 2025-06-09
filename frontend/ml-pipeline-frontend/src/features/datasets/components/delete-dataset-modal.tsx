import { Button } from '@/components/ui/button';
import { useDeleteDataset } from '@/services';
import { toast } from 'sonner';
import { Trash2 } from 'lucide-react';
import { DatasetSummary } from '@/types';
import DestructiveActionConfirmationModal from '@/components/common/destructive-action-confirmation-dialog';

export interface DeleteDatasetModalProps {
  projectId: string;
  dataset: DatasetSummary;
}

function DeleteDatasetModal(props: DeleteDatasetModalProps) {
  const { mutate: deleteDataset, isPending } = useDeleteDataset({
    onSuccess: () => toast.success('Successfully deleted dataset!'),
    onError: (error) => toast.error(error.error),
  });

  return (
    <DestructiveActionConfirmationModal
      dialogTrigger={
        <Button variant='ghost'>
          <Trash2 className='text-destructive hover:text-destructive/80' />
        </Button>
      }
      itemName={props.dataset.name}
      itemType='dataset'
      onConfirm={() =>
        deleteDataset({
          projectId: props.projectId,
          datasetId: props.dataset.id,
        })
      }
      isLoading={isPending}
    />
  );
}

export default DeleteDatasetModal;
