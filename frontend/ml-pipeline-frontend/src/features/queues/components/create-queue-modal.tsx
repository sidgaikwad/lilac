import { Button } from '@/components/ui/button';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog';
import { useCreateQueue } from '@/services/queues/create-queue.mutation';
import {
  CreateQueueForm,
  CreateQueueFormValues,
} from '../forms/create-queue-form';

export function CreateQueueModal() {
  const createQueueMutation = useCreateQueue({});

  const onSubmit = (values: CreateQueueFormValues) => {
    createQueueMutation.mutate(values);
  };

  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button>Create Queue</Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Create a new queue</DialogTitle>
          <DialogDescription>
            Queues are used to manage job scheduling and resource allocation.
          </DialogDescription>
        </DialogHeader>
        <CreateQueueForm
          onSubmit={onSubmit}
          isPending={createQueueMutation.isPending}
        />
      </DialogContent>
    </Dialog>
  );
}
