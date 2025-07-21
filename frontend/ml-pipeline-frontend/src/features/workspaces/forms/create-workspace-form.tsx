import { zodResolver } from '@hookform/resolvers/zod';
import { useForm, useFormContext } from 'react-hook-form';
import { z } from 'zod/v4';

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
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { Slider } from '@/components/ui/slider';
import { defineStepper } from '@stepperize/react';
import createFormStore from '@/store/use-form-data';
import { Separator } from '@/components/ui/separator';
import * as React from 'react';
import { RadioGroup } from '@/components/ui/radio-group';
import { EnvironmentCard } from '@/features/workspaces/components/environment-card';
import {
  mockEnvironments,
  mockComputeClusters,
} from '@/features/workspaces/mock-data';
import { JupyterIcon } from '@/icons/jupyter';
import { VSCodeIcon } from '@/icons/vscode';
import { RStudioIcon } from '@/icons/rstudio';
import { RayIcon } from '@/icons/ray';
import { ComputeClusterCard } from '@/features/workspaces/components/compute-cluster-card';
import { useListClusters } from '@/services/clusters';
import { EksLogo } from '@/icons/eks';
import { ClusterSummary } from '@/types';
import { Skeleton } from '@/components/ui/skeleton';

const environmentSchema = z.object({
  name: z.string().min(1, 'Workspace name is required'),
  environment: z.string(),
});

const clusterIdSchema = z.object({
  clusterId: z.string(),
});

const hardwareSchema = z.object({
  cpu: z.number().min(0.5).max(16),
  memory: z.number().min(1024).max(131072),
  preset: z.string().optional(),
});

const computeClusterSchema = z.object({
  computeCluster: z.string(),
});

export type CreateWorkspaceFormValues = z.infer<
  typeof environmentSchema &
    typeof clusterIdSchema &
    typeof hardwareSchema &
    typeof computeClusterSchema
>;

const useFormStore = createFormStore<CreateWorkspaceFormValues>();

const { useStepper, utils } = defineStepper(
  {
    id: 'environment',
    label: 'Environment',
    schema: environmentSchema,
  },
  {
    id: 'cluster',
    label: 'Cluster',
    schema: clusterIdSchema,
  },
  {
    id: 'hardware',
    label: 'Hardware',
    schema: hardwareSchema,
  },
  {
    id: 'compute',
    label: 'Distributed Compute',
    schema: computeClusterSchema,
  }
);

export interface CreateWorkspaceFormProps {
  onSubmit: (values: CreateWorkspaceFormValues) => void;
}

export function CreateWorkspaceForm({ onSubmit }: CreateWorkspaceFormProps) {
  const stepper = useStepper();
  const { formValues, setFormValues } = useFormStore((state) => ({
    formValues: state.formValues,
    setFormValues: state.setFormValues,
  }));

  const form = useForm({
    mode: 'onChange',
    resolver: zodResolver(stepper.current.schema),
  });

  const onStepSubmit = (values: z.infer<typeof stepper.current.schema>) => {
    setFormValues(values);
    if (stepper.isLast) {
      onSubmit({ ...formValues, ...values } as CreateWorkspaceFormValues);
      stepper.reset();
    } else {
      stepper.next();
    }
  };

  const currentIndex = utils.getIndex(stepper.current.id);

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onStepSubmit)}>
        <nav aria-label='Workspace Creation Steps' className='group my-4'>
          <ol className='flex flex-col gap-2' aria-orientation='vertical'>
            {stepper.all.map((step, index, array) => (
              <React.Fragment key={step.id}>
                <li className='flex flex-shrink-0 items-center gap-4'>
                  <Button
                    type='button'
                    role='tab'
                    variant={index <= currentIndex ? 'default' : 'secondary'}
                    aria-current={
                      stepper.current.id === step.id ? 'step' : undefined
                    }
                    aria-posinset={index + 1}
                    aria-setsize={stepper.all.length}
                    aria-selected={stepper.current.id === step.id}
                    className='flex size-10 items-center justify-center rounded-full'
                    onClick={() => {
                      if (index <= currentIndex) {
                        stepper.goTo(step.id);
                      }
                    }}
                  >
                    {index + 1}
                  </Button>
                  <span className='text-sm font-medium'>{step.label}</span>
                </li>
                <div className='flex gap-4'>
                  <div
                    className='flex justify-center'
                    style={{
                      paddingInlineStart: '1.25rem',
                    }}
                  >
                    {index < array.length - 1 && (
                      <Separator
                        orientation='vertical'
                        className={`h-full w-[1px] ${
                          index < currentIndex
                            ? 'bg-accent'
                            : 'bg-accent-border'
                        }`}
                      />
                    )}
                  </div>
                  <div className='my-4 flex-1'>
                    {stepper.current.id === step.id &&
                      stepper.switch({
                        environment: () => <EnvironmentStep />,
                        cluster: () => <ClusterStep />,
                        hardware: () => <HardwareStep />,
                        compute: () => <ComputeStep />,
                      })}
                  </div>
                </div>
              </React.Fragment>
            ))}
          </ol>
        </nav>
        <div className='space-y-4'>
          <div className='flex justify-end gap-4'>
            <Button
              variant='secondary'
              onClick={stepper.prev}
              disabled={stepper.isFirst}
              type='button'
            >
              Back
            </Button>
            <Button type='submit'>{stepper.isLast ? 'Submit' : 'Next'}</Button>
          </div>
        </div>
      </form>
    </Form>
  );
}

function EnvironmentStep() {
  const { control, register } = useFormContext<CreateWorkspaceFormValues>();

  const getIcon = (icon: string) => {
    switch (icon) {
      case 'jupyter-icon':
        return <JupyterIcon className='size-24' />;
      case 'vscode-icon':
        return <VSCodeIcon className='size-24' />;
      case 'rstudio-icon':
        return <RStudioIcon className='size-24' />;
      default:
        return null;
    }
  };

  return (
    <div className='space-y-8'>
      <FormField
        control={control}
        name='name'
        render={({ field }) => (
          <FormItem>
            <FormLabel>Workspace Name</FormLabel>
            <FormControl>
              <Input placeholder='andrea_lowe_s_sas_viya_session' {...field} />
            </FormControl>
            <FormMessage />
          </FormItem>
        )}
      />
      <FormField
        name={register('environment').name}
        control={control}
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
    </div>
  );
}

function ClusterStep() {
  const { control, register } = useFormContext<CreateWorkspaceFormValues>();
  const { data: clusters, isLoading } = useListClusters();

  const getIcon = (cluster: ClusterSummary) => {
    switch (cluster.clusterType) {
      case 'aws_eks':
        return <EksLogo className="size-12" />;
      default:
        return null;
    }
  };

  if (isLoading) {
    return (
      <div className="grid grid-cols-2 gap-4">
        {Array.from({ length: 2 }).map((_, i) => (
          <Skeleton key={i} className="h-32 w-full" />
        ))}
      </div>
    );
  }

  return (
    <FormField
      name={register('clusterId').name}
      control={control}
      render={({ field }) => (
        <FormItem className="space-y-3">
          <FormMessage />
          <FormControl>
            <RadioGroup
              onValueChange={field.onChange}
              defaultValue={field.value}
              className="grid grid-cols-2 gap-4"
            >
              {clusters?.map((cluster) => (
                <EnvironmentCard
                  key={cluster.clusterId}
                  icon={getIcon(cluster)}
                  title={cluster.clusterName}
                  description={cluster.clusterDescription || ''}
                  value={cluster.clusterId}
                  className="h-full w-full"
                />
              ))}
            </RadioGroup>
          </FormControl>
        </FormItem>
      )}
    />
  );
}

function HardwareStep() {
  const { control, watch, setValue } =
    useFormContext<CreateWorkspaceFormValues>();
  const selectedPreset = watch('preset');

  const presets = [
    { name: 'Extra Small', cpu: 1, memory: 2048 },
    { name: 'Small', cpu: 2, memory: 4096 },
    { name: 'Medium', cpu: 4, memory: 8192 },
    { name: 'Large', cpu: 8, memory: 16384 },
    { name: 'Extra Large', cpu: 16, memory: 32768 },
    { name: 'Custom', cpu: 0, memory: 0 },
  ];

  const handlePresetChange = (presetName: string) => {
    setValue('preset', presetName);
    const preset = presets.find((p) => p.name === presetName);
    if (preset && preset.name !== 'Custom') {
      setValue('cpu', preset.cpu);
      setValue('memory', preset.memory);
    }
  };

  return (
    <div className="space-y-8">
      <FormField
        control={control}
        name="preset"
        render={({ field }) => (
          <FormItem>
            <FormLabel>Preset</FormLabel>
            <Select
              onValueChange={handlePresetChange}
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
      {selectedPreset === 'Custom' && (
        <>
          <FormField
            control={control}
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
            control={control}
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
    </div>
  );
}

function ComputeStep() {
  const { control, register } = useFormContext<CreateWorkspaceFormValues>();
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

  const getIcon = (icon: string) => {
    switch (icon) {
      case 'ray-icon':
        return <RayIcon className='size-12' />;
      case 'none-icon':
        return null;
      default:
        return null;
    }
  };

  return (
    <FormField
      name={register('computeCluster').name}
      control={control}
      render={({ field }) => (
        <FormItem className='space-y-3'>
          <FormMessage />
          <FormControl>
            <RadioGroup
              onValueChange={field.onChange}
              defaultValue={field.value || 'None'}
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
  );
}
