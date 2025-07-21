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
import { useCreateWorkspace } from '@/services/workspaces';

export interface CreateWorkspaceModalProps {
  isOpen: boolean;
  setOpen: (isOpen: boolean) => void;
  projectId: string;
}

const CreateWorkspaceModal: React.FC<CreateWorkspaceModalProps> = (
  props: CreateWorkspaceModalProps
) => {
  const createWorkspace = useCreateWorkspace({
    onSuccess: (data) => {
      console.log('Workspace created successfully:', data);
      toast.success('Successfully created workspace!');
      props.setOpen(false);
    },
    onError: (error) => {
      console.error('Failed to create workspace:', error);
      toast.error('Failed to create workspace');
    },
  });

  const onSubmit = async (data: CreateWorkspaceFormValues) => {
    const imageName =
      data.environment.toLowerCase() === 'jupyterlab'
        ? 'jupyter-lilac:latest'
        : '';

    createWorkspace.mutate({
      projectId: props.projectId,
      payload: {
        name: data.name || 'hardcoded-workspace',
        cluster_id: data.clusterId,
        ide: data.environment.toLowerCase() as 'jupyterlab' | 'vscode' | 'rstudio',
        image: imageName,
        cpu_millicores: data.cpu * 1000,
        memory_mb: data.memory,
        gpu: data.gpu,
      },
    });
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
