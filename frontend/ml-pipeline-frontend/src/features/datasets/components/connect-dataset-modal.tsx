import {
  Dialog,
  DialogTrigger,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
} from '@/components/ui/dialog';
import { Button } from '@/components/ui/button';
import { useCreateDataset } from '@/services';
import { toast } from 'sonner';
import ConnectDatasetForm, {
  DataSetFormValues,
} from '../forms/connect-dataset-form';

export interface ConnectDatasetModalProps {
  isOpen: boolean;
  setOpen: (isOpen: boolean) => void;
  projectId: string;
}

const ConnectDatasetModal: React.FC<ConnectDatasetModalProps> = (
  props: ConnectDatasetModalProps
) => {
  const { mutate: createDataset } = useCreateDataset({
    onSuccess: (_data) => toast.success('Successfully conneced dataset!'),
    onError: (error) => toast.error(error.error),
  });

  const onSubmit = async (data: DataSetFormValues) => {
    createDataset({
      datasetName: data.datasetName,
      projectId: props.projectId,
      description: data.datasetDescription,
      source: data.source,
    });
    props.setOpen(false);
  };

  return (
    <Dialog open={props.isOpen} onOpenChange={props.setOpen}>
      <DialogTrigger asChild>
        <Button>Connect Dataset</Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Connect Dataset</DialogTitle>
          <DialogDescription></DialogDescription>
        </DialogHeader>

        <ConnectDatasetForm projectId={props.projectId} onSubmit={onSubmit} />
      </DialogContent>
    </Dialog>
  );
};

export default ConnectDatasetModal;
