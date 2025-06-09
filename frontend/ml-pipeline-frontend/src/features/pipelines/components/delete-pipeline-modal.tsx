import { Button } from '@/components/ui/button';
import { useDeletePipeline } from '@/services';
import { toast } from 'sonner';
import { Trash2 } from 'lucide-react';
import { PipelineSummary } from '@/types';
import DestructiveActionConfirmationModal from '@/components/common/destructive-action-confirmation-dialog';

export interface DeletePipelineModalProps {
  projectId: string;
  pipeline: PipelineSummary;
}

function DeletePipelineModal(props: DeletePipelineModalProps) {
  const { mutate: deletePipeline, isPending } = useDeletePipeline({
    onSuccess: () => toast.success('Successfully deleted pipeline!'),
    onError: (error) => toast.error(error.error),
  });

  return (
    <DestructiveActionConfirmationModal
      dialogTrigger={
        <Button variant='ghost'>
          <Trash2 className='text-destructive hover:text-destructive/80' />
        </Button>
      }
      itemName={props.pipeline.name}
      itemType='pipeline'
      onConfirm={() =>
        deletePipeline({
          projectId: props.projectId,
          pipelineId: props.pipeline.id,
        })
      }
      isLoading={isPending}
    />
  );
}

export default DeletePipelineModal;
