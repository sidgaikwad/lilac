import { zodResolver } from '@hookform/resolvers/zod';
import { useForm } from 'react-hook-form';
import { z } from 'zod';
import { Button } from '@/components/ui/button';
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
import { Checkbox } from '@/components/ui/checkbox';

const formSchema = z.object({
  name: z.string().min(1, 'Queue name is required'),
  priority: z.coerce.number().int().min(0),
  clusterTargets: z
    .array(z.string())
    .min(1, 'At least one cluster must be selected'),
});

export type CreateQueueFormValues = z.infer<typeof formSchema>;

export interface CreateQueueFormProps {
  onSubmit: (values: CreateQueueFormValues) => void;
  isPending: boolean;
}

export function CreateQueueForm({ onSubmit, isPending }: CreateQueueFormProps) {
  const { data: clusters, isLoading: isLoadingClusters } = useListClusters();

  const form = useForm<CreateQueueFormValues>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      name: '',
      priority: 0,
      clusterTargets: [],
    },
  });

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className='space-y-8'>
        <FormField
          control={form.control}
          name='name'
          render={({ field }) => (
            <FormItem>
              <FormLabel>Queue Name</FormLabel>
              <FormControl>
                <Input placeholder='e.g. H100-queue' {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name='priority'
          render={({ field }) => (
            <FormItem>
              <FormLabel>Priority</FormLabel>
              <FormControl>
                <Input type='number' {...field} />
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
          name='clusterTargets'
          render={() => (
            <FormItem>
              <div className='mb-4'>
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
                    name='clusterTargets'
                    render={({ field }) => {
                      return (
                        <FormItem
                          key={cluster.clusterId}
                          className='flex flex-row items-start space-y-0 space-x-3'
                        >
                          <FormControl>
                            <Checkbox
                              checked={field.value?.includes(cluster.clusterId)}
                              onCheckedChange={(checked) => {
                                return checked
                                  ? field.onChange([
                                      ...field.value,
                                      cluster.clusterId,
                                    ])
                                  : field.onChange(
                                      field.value?.filter(
                                        (value) => value !== cluster.clusterId
                                      )
                                    );
                              }}
                            />
                          </FormControl>
                          <FormLabel className='font-normal'>
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
        <Button type='submit' disabled={isPending}>
          {isPending ? 'Creating...' : 'Create'}
        </Button>
      </form>
    </Form>
  );
}
