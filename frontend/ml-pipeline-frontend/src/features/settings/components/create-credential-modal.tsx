import {
  Dialog,
  DialogTrigger,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
} from '@/components/ui/dialog';
import { Button } from '@/components/ui/button';
import { useCreateCredential } from '@/services';
import { toast } from '@/components/toast';
import {
  CreateCredentialForm,
  CredentialTypeFormValues,
} from '../forms/create-credential-form';

export interface ConnectCredentialModalProps {
  isOpen: boolean;
  setOpen: (isOpen: boolean) => void;
}

export function CreateCredentialModal(props: ConnectCredentialModalProps) {
  const { mutate: createCredential } = useCreateCredential({
    onSuccess: (_data) => toast.success('Successfully created credentials!'),
    onError: (error) => toast.error(error.error),
  });

  const onSubmit = async (data: CredentialTypeFormValues) => {
    createCredential({
      credentialName: data.credentialName,
      credentialDescription: data.credentialDescription,
      credentials: data.credentials,
    });
    props.setOpen(false);
  };

  return (
    <Dialog open={props.isOpen} onOpenChange={props.setOpen}>
      <DialogTrigger asChild>
        <Button>Create Credentials</Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Connect Credential</DialogTitle>
          <DialogDescription></DialogDescription>
        </DialogHeader>

        <CreateCredentialForm onSubmit={onSubmit} />
      </DialogContent>
    </Dialog>
  );
}

export default CreateCredentialModal;
