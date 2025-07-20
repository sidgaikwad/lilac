import {
  Dialog,
  DialogTrigger,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
} from '@/components/ui/dialog';
import { Button } from '@/components/ui/button';
import { useCreateCluster } from '@/services';
import { toast } from '@/components/toast';
import {
  CreateClusterForm,
  ClusterTypeFormValues,
} from '../forms/create-cluster-form';

export interface CreateClusterModalProps {
  isOpen: boolean;
  setOpen: (isOpen: boolean) => void;
}

export function CreateClusterModal(props: CreateClusterModalProps) {
  const { mutate: createCluster } = useCreateCluster({
    onSuccess: (_data) => toast.success('Successfully created cluster!'),
    onError: (error) => toast.error(error.error),
  });

  const onSubmit = async (data: ClusterTypeFormValues) => {
    createCluster({
      clusterName: data.clusterName,
      clusterDescription: data.clusterDescription,
      clusterConfig: data.clusterConfig,
      credentialId: data.credentialId,
    });
    props.setOpen(false);
  };

  return (
    <Dialog open={props.isOpen} onOpenChange={props.setOpen}>
      <DialogTrigger asChild>
        <Button>Create Cluster</Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Create Cluster</DialogTitle>
          <DialogDescription></DialogDescription>
        </DialogHeader>

        <CreateClusterForm onSubmit={onSubmit} />
      </DialogContent>
    </Dialog>
  );
}

export default CreateClusterModal;
