import * as RadioGroupPrimitive from '@radix-ui/react-radio-group';
import { cn } from '@/lib/utils';
import { useFormContext } from 'react-hook-form';
import { CreateWorkspaceFormValues } from '../forms/create-workspace-form';
import {
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '@/components/ui/form';

const clusterOptions = [
  { id: 'none', name: 'None' },
  { id: 'ray', name: 'Ray' },
  { id: 'spark', name: 'Spark' },
  { id: 'slurm', name: 'Slurm' },
];

export function ComputeClusterOptions() {
  const { control } = useFormContext<CreateWorkspaceFormValues>();

  return (
    <div className='space-y-8'>
      <FormField
        control={control}
        name='computeCluster'
        render={({ field }) => (
          <FormItem className='space-y-3'>
            <FormLabel>Attach Compute Cluster</FormLabel>
            <FormMessage />
            <FormControl>
              <RadioGroupPrimitive.Root
                onValueChange={field.onChange}
                defaultValue={field.value}
                className='grid grid-cols-4 gap-4'
              >
                {clusterOptions.map((option) => (
                  <FormItem key={option.id} className='flex items-center gap-3'>
                    <RadioGroupPrimitive.Item
                      value={option.id}
                      className={cn(
                        'group relative rounded-xl text-start',
                        'data-[state=checked]:ring-accent-border-hover data-[state=checked]:ring-2'
                      )}
                    >
                      <div className='px-4 py-2'>{option.name}</div>
                    </RadioGroupPrimitive.Item>
                  </FormItem>
                ))}
              </RadioGroupPrimitive.Root>
            </FormControl>
          </FormItem>
        )}
      />
      <div className='mt-8 flex flex-col items-center justify-center text-center'>
        <div className='text-6xl'>⚛️</div>
        <h3 className='mt-4 text-2xl font-semibold'>Compute Clusters</h3>
        <p className='text-muted-foreground mt-2'>
          Lilac managed compute clusters provide additional distributed
          computational power for workspaces and jobs.
        </p>
        <p className='mt-4 text-sm'>
          Note: First choose an execution environment that has the corresponding
          client libraries for the cluster you desire.
        </p>
        <a href='#' className='text-primary mt-2 text-sm hover:underline'>
          Learn about Compute Clusters in our docs
        </a>
      </div>
    </div>
  );
}
