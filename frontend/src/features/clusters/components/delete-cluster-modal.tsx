import { Button } from '@/components/ui/button';
import { useDeleteCluster } from '@/services';
import { toast } from '@/components/toast';
import { Trash2 } from 'lucide-react';
import { ClusterSummary } from '@/types';
import DestructiveActionConfirmationModal from '@/components/common/destructive-action-confirmation-dialog';

export interface DeleteClusterModalProps {
  cluster: ClusterSummary;
}

function DeleteClusterModal(props: DeleteClusterModalProps) {
  const { mutate: deleteCluster, isPending } = useDeleteCluster({
    onSuccess: () => toast.success('Successfully deleted cluster!'),
    onError: (error) => toast.error(error.error),
  });

  return (
    <DestructiveActionConfirmationModal
      dialogTrigger={
        <Button variant='ghost' type='button'>
          <Trash2 className='text-destructive hover:text-destructive-hover' />
        </Button>
      }
      itemName={props.cluster.clusterName}
      itemType='cluster'
      onConfirm={() =>
        deleteCluster({
          clusterId: props.cluster.clusterId,
        })
      }
      isLoading={isPending}
    />
  );
}

export default DeleteClusterModal;
