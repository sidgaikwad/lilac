import {
  Dialog,
  DialogTrigger,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
} from '@/components/ui/dialog';
import { Button } from '@/components/ui/button';
import { toast } from '@/components/toast';
import {
  CreateWorkspaceForm,
  CreateWorkspaceFormValues,
} from '../forms/create-workspace-form';

export interface CreateWorkspaceModalProps {
  isOpen: boolean;
  setOpen: (isOpen: boolean) => void;
  projectId: string;
}

const CreateWorkspaceModal: React.FC<CreateWorkspaceModalProps> = (
  props: CreateWorkspaceModalProps
) => {
  const onSubmit = async (data: CreateWorkspaceFormValues) => {
    console.log('TODO: Call create workspace mutation', data);
    toast.success('Successfully created workspace!');
    props.setOpen(false);
  };

  return (
    <Dialog open={props.isOpen} onOpenChange={props.setOpen}>
      <DialogTrigger asChild>
        <Button>Create Workspace</Button>
      </DialogTrigger>
      <DialogContent className='overflow-y-auto'>
        <DialogHeader>
          <DialogTitle>Create Workspace</DialogTitle>
          <DialogDescription></DialogDescription>
        </DialogHeader>
        <CreateWorkspaceForm onSubmit={onSubmit} />
      </DialogContent>
    </Dialog>
  );
};

export default CreateWorkspaceModal;
