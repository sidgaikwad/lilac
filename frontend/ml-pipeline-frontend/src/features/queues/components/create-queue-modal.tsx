import { zodResolver } from '@hookform/resolvers/zod';
import { useForm } from 'react-hook-form';
import { z } from 'zod';
import { Button } from '@/components/ui/button';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog';
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '@/components/ui/form';
import { Input } from '@/components/ui/input';
import { useListClusters } from '@/services/clusters/list-clusters.query';
import { useCreateQueue } from '@/services/queues/create-queue.mutation';
import { Checkbox } from '@/components/ui/checkbox';

const formSchema = z.object({
  name: z.string().min(1, 'Queue name is required'),
  priority: z.coerce.number().int().min(0),
  clusterTargets: z.array(z.string()).min(1, 'At least one cluster must be selected'),
});

type FormValues = z.infer<typeof formSchema>;

export function CreateQueueModal() {
  const { data: clusters, isLoading: isLoadingClusters } = useListClusters();
  const createQueueMutation = useCreateQueue({});

  const form = useForm<FormValues>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      name: '',
      priority: 0,
      clusterTargets: [],
    },
  });

  const onSubmit = (values: FormValues) => {
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
        <Form {...form}>
          <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8">
            <FormField
              control={form.control}
              name="name"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Queue Name</FormLabel>
                  <FormControl>
                    <Input placeholder="e.g. gpu-queue" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="priority"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Priority</FormLabel>
                  <FormControl>
                    <Input type="number" {...field} />
                  </FormControl>
                  <FormDescription>
                    Higher numbers have higher priority.
                  </FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="clusterTargets"
              render={() => (
                <FormItem>
                  <div className="mb-4">
                    <FormLabel>Cluster Targets</FormLabel>
                    <FormDescription>
                      Select the clusters that this queue can schedule jobs on.
                    </FormDescription>
                  </div>
                  {isLoadingClusters ? (
                    <p>Loading clusters...</p>
                  ) : (
                    clusters?.map((cluster) => (
                      <FormField
                        key={cluster.clusterId}
                        control={form.control}
                        name="clusterTargets"
                        render={({ field }) => {
                          return (
                            <FormItem
                              key={cluster.clusterId}
                              className="flex flex-row items-start space-x-3 space-y-0"
                            >
                              <FormControl>
                                <Checkbox
                                  checked={field.value?.includes(
                                    cluster.clusterId
                                  )}
                                  onCheckedChange={(checked) => {
                                    return checked
                                      ? field.onChange([
                                          ...field.value,
                                          cluster.clusterId,
                                        ])
                                      : field.onChange(
                                          field.value?.filter(
                                            (value) =>
                                              value !== cluster.clusterId
                                          )
                                        );
                                  }}
                                />
                              </FormControl>
                              <FormLabel className="font-normal">
                                {cluster.clusterName}
                              </FormLabel>
                            </FormItem>
                          );
                        }}
                      />
                    ))
                  )}
                  <FormMessage />
                </FormItem>
              )}
            />
            <Button type="submit" disabled={createQueueMutation.isPending}>
              {createQueueMutation.isPending ? 'Creating...' : 'Create'}
            </Button>
          </form>
        </Form>
      </DialogContent>
    </Dialog>
  );
}