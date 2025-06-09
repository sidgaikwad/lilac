import { Button } from '@/components/ui/button';
import { useDeleteOrganization } from '@/services';
import { toast } from 'sonner';
import { Trash2 } from 'lucide-react';
import { Organization } from '@/types';
import DestructiveActionConfirmationModal from '@/components/common/destructive-action-confirmation-dialog';

export interface DeleteOrganizationModalProps {
  organization: Organization;
}

function DeleteOrganizationModal(props: DeleteOrganizationModalProps) {
  const { mutate: deleteOrg, isPending } = useDeleteOrganization({
    onSuccess: () => toast.success('Successfully deleted organization!'),
    onError: (error) => toast.error(error.error),
  });

  return (
    <DestructiveActionConfirmationModal
      dialogTrigger={
        <Button variant='ghost'>
          <Trash2 className='text-destructive hover:text-destructive/80' />
        </Button>
      }
      itemName={props.organization.name}
      itemType='organization'
      onConfirm={() => deleteOrg({ organizationId: props.organization.id })}
      isLoading={isPending}
    />
  );
}

export default DeleteOrganizationModal;
