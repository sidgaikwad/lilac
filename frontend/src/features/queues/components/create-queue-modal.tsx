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
import { toast } from '@/components/toast';
import { useState } from 'react';

export function CreateQueueModal() {
  const [open, setOpen] = useState(false);
  const createQueueMutation = useCreateQueue({
    onSuccess: (_data) => {
      toast.success('Successfully created queue!');
      setOpen(false);
    },
    onError: (error) =>
      toast.error('Error', {
        description: error.error,
      }),
  });

  const onSubmit = (values: CreateQueueFormValues) => {
    createQueueMutation.mutate(values);
  };

  return (
    <Dialog open={open} onOpenChange={setOpen}>
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
