import { zodResolver } from '@hookform/resolvers/zod';
import { useForm } from 'react-hook-form';
import { z } from 'zod';

import { Button } from '@/components/ui/button';
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '@/components/ui/form';
import { Input } from '@/components/ui/input';
import { Slider } from '@/components/ui/slider';
import { RadioGroup } from '@/components/ui/radio-group';
import { EnvironmentCard } from '@/features/workspaces/components/environment-card';
import {
  mockEnvironments,
  mockComputeClusters,
  Workspace,
} from '@/features/workspaces/mock-data';
import { JupyterIcon } from '@/components/icons/jupyter';
import { VSCodeIcon } from '@/components/icons/vscode';
import { SlurmIcon } from '@/components/icons/slurm';
import { RayIcon } from '@/components/icons/ray';
import { ComputeClusterCard } from '@/features/workspaces/components/compute-cluster-card';
import { Alert } from '@/components/common/alert';
import * as React from 'react';

const editWorkspaceSchema = z.object({
  name: z.string().optional(),
  environment: z.string(),
  cpu: z.number().min(0.5).max(16),
  memory: z.number().min(1).max(128),
  computeCluster: z.string(),
});

export type EditWorkspaceFormValues = z.infer<typeof editWorkspaceSchema>;

export interface EditWorkspaceFormProps {
  workspace: Workspace;
  onSubmit: (values: EditWorkspaceFormValues) => void;
  onCancel: () => void;
}

export function EditWorkspaceForm({
  workspace,
  onSubmit,
  onCancel,
}: EditWorkspaceFormProps) {
  const form = useForm<EditWorkspaceFormValues>({
    resolver: zodResolver(editWorkspaceSchema),
    defaultValues: {
      name: workspace.name,
      environment: workspace.environment.name,
      cpu: workspace.hardware.cpu,
      memory: workspace.hardware.memory,
      computeCluster: workspace.hardware.tier,
    },
  });

  const getIcon = (icon: string) => {
    switch (icon) {
      case 'jupyter-icon':
        return <JupyterIcon className='size-24' />;
      case 'vscode-icon':
        return <VSCodeIcon className='size-24' />;
      case 'slurm-icon':
        return <SlurmIcon className='size-12' />;
      case 'ray-icon':
        return <RayIcon className='size-12' />;
      default:
        return null;
    }
  };

  const [maxWidth, setMaxWidth] = React.useState(0);
  const refs = React.useRef<(HTMLDivElement | null)[]>([]);

  React.useEffect(() => {
    let max = 0;
    refs.current.forEach((ref) => {
      if (ref) {
        max = Math.max(max, ref.offsetWidth);
      }
    });
    setMaxWidth(max);
  }, [mockComputeClusters]);

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className='space-y-8'>
        <FormField
          control={form.control}
          name='name'
          render={({ field }) => (
            <FormItem>
              <FormLabel>Workspace Name (optional)</FormLabel>
              <FormControl>
                <Input
                  placeholder='andrea_lowe_s_sas_viya_session'
                  {...field}
                />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />

        <FormField
          name='environment'
          control={form.control}
          render={({ field }) => (
            <FormItem className='space-y-3'>
              <FormLabel>Environment</FormLabel>
              <FormMessage />
              <FormControl>
                <RadioGroup
                  onValueChange={field.onChange}
                  defaultValue={field.value}
                  className='grid grid-cols-2 gap-4'
                >
                  {mockEnvironments.map((env) => (
                    <EnvironmentCard
                      key={env.name}
                      icon={getIcon(env.icon)}
                      title={env.name}
                      description={env.description}
                      value={env.name}
                    />
                  ))}
                </RadioGroup>
              </FormControl>
            </FormItem>
          )}
        />

        <FormField
          control={form.control}
          name='cpu'
          render={({ field }) => (
            <FormItem>
              <FormLabel>CPU (vCPUs)</FormLabel>
              <div className='flex items-center gap-4'>
                <FormControl>
                  <Slider
                    min={0.5}
                    max={16}
                    step={0.5}
                    value={[field.value]}
                    onValueChange={(value) => field.onChange(value[0])}
                  />
                </FormControl>
                <span className='w-16 text-right'>{field.value}</span>
              </div>
              <FormMessage />
            </FormItem>
          )}
        />

        <FormField
          control={form.control}
          name='memory'
          render={({ field }) => (
            <FormItem>
              <FormLabel>Memory (GiB)</FormLabel>
              <div className='flex items-center gap-4'>
                <FormControl>
                  <Slider
                    min={1}
                    max={128}
                    step={1}
                    value={[field.value]}
                    onValueChange={(value) => field.onChange(value[0])}
                  />
                </FormControl>
                <span className='w-16 text-right'>{field.value}</span>
              </div>
              <FormMessage />
            </FormItem>
          )}
        />

        <FormField
          name='computeCluster'
          control={form.control}
          render={({ field }) => (
            <FormItem className='space-y-3'>
              <FormLabel>Compute Cluster</FormLabel>
              <FormMessage />
              <FormControl>
                <RadioGroup
                  onValueChange={field.onChange}
                  defaultValue={field.value}
                  className='grid grid-cols-3 gap-4'
                >
                  {mockComputeClusters.map((cluster, i) => (
                    <div
                      key={cluster.name}
                      ref={(el) => {
                        refs.current[i] = el;
                      }}
                      style={{ minWidth: maxWidth }}
                    >
                      <ComputeClusterCard
                        icon={getIcon(cluster.icon)}
                        title={cluster.name}
                        description={cluster.description}
                        value={cluster.name}
                      />
                    </div>
                  ))}
                </RadioGroup>
              </FormControl>
            </FormItem>
          )}
        />

        {workspace.status === 'Running' && (
          <Alert
            variant='warn'
            title='Restart Required'
            description='Changing these settings will restart the workspace.'
          />
        )}

        <div className='flex justify-end gap-4'>
          <Button type='button' variant='secondary' onClick={onCancel}>
            Cancel
          </Button>
          <Button type='submit'>Save</Button>
        </div>
      </form>
    </Form>
  );
}