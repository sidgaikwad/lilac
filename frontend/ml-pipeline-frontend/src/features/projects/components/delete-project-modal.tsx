import { Button } from '@/components/ui/button';
import { useDeleteProject } from '@/services';
import { toast } from '@/components/toast';
import { Trash2 } from 'lucide-react';
import { Project } from '@/types';
import DestructiveActionConfirmationModal from '@/components/common/destructive-action-confirmation-dialog';

export interface DeleteProjectModalProps {
  project: Project;
}

function DeleteProjectModal(props: DeleteProjectModalProps) {
  const { mutate: deleteProject, isPending } = useDeleteProject({
    onSuccess: () => toast.success('Successfully deleted project!'),
    onError: (error) => toast.error(error.error),
  });

  return (
    <DestructiveActionConfirmationModal
      dialogTrigger={
        <Button variant='ghost'>
          <Trash2 className='text-destructive hover:text-destructive/80' />
        </Button>
      }
      itemName={props.project.projectName}
      itemType='project'
      onConfirm={() =>
        deleteProject({
          projectId: props.project.projectId,
        })
      }
      isLoading={isPending}
    />
  );
}

export default DeleteProjectModal;
