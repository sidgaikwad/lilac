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
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { mockComputeClusters } from '@/features/workspaces/mock-data';
import { Workspace } from '@/types/api/workspace';
import { ComputeClusterCard } from '@/features/workspaces/components/compute-cluster-card';
import { Alert } from '@/components/common/alert';
import * as React from 'react';
import { RayIcon, SlurmIcon } from '@/icons';

const editWorkspaceSchema = z.object({
  name: z.string().optional(),
  cpu: z.number().min(0.5).max(16),
  memory: z.number().min(1024).max(131072),
  preset: z.string().optional(),
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
  const presets = [
    { name: 'Extra Small', cpu: 1, memory: 2048 },
    { name: 'Small', cpu: 2, memory: 4096 },
    { name: 'Medium', cpu: 4, memory: 8192 },
    { name: 'Large', cpu: 8, memory: 16384 },
    { name: 'Extra Large', cpu: 16, memory: 32768 },
    { name: 'Custom', cpu: 0, memory: 0 },
  ];
  const form = useForm<EditWorkspaceFormValues>({
    resolver: zodResolver(editWorkspaceSchema),
    defaultValues: {
      name: workspace.name,
      cpu: workspace.cpu_millicores / 1000,
      memory: workspace.memory_mb,
      // TODO: get computeCluster from workspace
      computeCluster: 'None',
    },
  });

  const getIcon = (icon: string) => {
    switch (icon) {
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
          control={form.control}
          name="preset"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Preset</FormLabel>
              <Select
                onValueChange={(value: string) => {
                  field.onChange(value);
                  const preset = presets.find((p) => p.name === value);
                  if (preset && preset.name !== 'Custom') {
                    form.setValue('cpu', preset.cpu);
                    form.setValue('memory', preset.memory);
                  }
                }}
                defaultValue={field.value}
              >
                <FormControl>
                  <SelectTrigger>
                    <SelectValue placeholder="Select a preset" />
                  </SelectTrigger>
                </FormControl>
                <SelectContent>
                  {presets.map((preset) => (
                    <SelectItem key={preset.name} value={preset.name}>
                      {preset.name !== 'Custom'
                        ? `${preset.name} (${preset.cpu} vCPUs, ${
                            preset.memory / 1024
                          } GB RAM)`
                        : 'Custom'}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
              <FormMessage />
            </FormItem>
          )}
        />
        {form.watch('preset') === 'Custom' && (
          <>
            <FormField
              control={form.control}
              name="cpu"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>CPU (vCPUs)</FormLabel>
                  <div className="flex items-center gap-4">
                    <FormControl>
                      <Slider
                        min={0.5}
                        max={16}
                        step={0.5}
                        value={[field.value]}
                        onValueChange={(value) => field.onChange(value[0])}
                      />
                    </FormControl>
                    <Input
                      type="number"
                      className="w-24"
                      value={field.value}
                      onChange={(e) =>
                        field.onChange(parseFloat(e.target.value))
                      }
                    />
                  </div>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="memory"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Memory (MiB)</FormLabel>
                  <div className="flex items-center gap-4">
                    <FormControl>
                      <Slider
                        min={1024}
                        max={131072}
                        step={1024}
                        value={[field.value]}
                        onValueChange={(value) => field.onChange(value[0])}
                      />
                    </FormControl>
                    <Input
                      type="number"
                      className="w-24"
                      value={field.value}
                      onChange={(e) =>
                        field.onChange(parseInt(e.target.value))
                      }
                    />
                  </div>
                  <FormMessage />
                </FormItem>
              )}
            />
          </>
        )}

        <FormField
          name="computeCluster"
          control={form.control}
          render={({ field }) => (
            <FormItem className="space-y-3">
              <FormLabel>Distributed Compute</FormLabel>
              <FormMessage />
              <FormControl>
                <RadioGroup
                  onValueChange={field.onChange}
                  defaultValue={field.value}
                  className="grid grid-cols-3 gap-4"
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

        {workspace.status === 'running' && (
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
