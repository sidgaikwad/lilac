import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import { toast } from '@/components/toast';
import {
  EditWorkspaceForm,
  EditWorkspaceFormValues,
} from '../forms/edit-workspace-form';
import { Workspace } from '../mock-data';
import { Badge } from '@/components/ui/badge';

export interface EditWorkspaceModalProps {
  isOpen: boolean;
  setOpen: (isOpen: boolean) => void;
  workspace?: Workspace;
}

const statusVariant: Record<
  Workspace['status'],
  'default' | 'destructive' | 'secondary' | 'outline'
> = {
  Running: 'default',
  Stopped: 'secondary',
  Starting: 'outline',
  Error: 'destructive',
};

export function EditWorkspaceModal({
  isOpen,
  setOpen,
  workspace,
}: EditWorkspaceModalProps) {
  if (!workspace) {
    return null;
  }

  const onSubmit = async (data: EditWorkspaceFormValues) => {
    console.log('TODO: Call update workspace mutation', data);
    toast.success('Successfully updated workspace!');
    setOpen(false);
  };

  return (
    <Dialog open={isOpen} onOpenChange={setOpen}>
      <DialogContent className="overflow-y-auto">
        <DialogHeader className="flex-row items-center justify-between">
          <DialogTitle>{workspace.name}</DialogTitle>
          <Badge variant={statusVariant[workspace.status]}>
            {workspace.status}
          </Badge>
        </DialogHeader>
        <EditWorkspaceForm
          workspace={workspace}
          onSubmit={onSubmit}
          onCancel={() => setOpen(false)}
        />
      </DialogContent>
    </Dialog>
  );
}