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
import { ComputeClusterCard } from '@/features/workspaces/components/compute-cluster-card';
import { JupyterIcon, VSCodeIcon, SlurmIcon, RayIcon } from '@/icons';

const environmentSchema = z.object({
  name: z.string().optional(),
  environment: z.string(),
});

const hardwareSchema = z.object({
  cpu: z.number().min(0.5).max(16),
  memory: z.number().min(1).max(128),
});

const computeClusterSchema = z.object({
  computeCluster: z.string(),
});

export type CreateWorkspaceFormValues = z.infer<
  typeof environmentSchema & typeof hardwareSchema & typeof computeClusterSchema
>;

const useFormStore = createFormStore<CreateWorkspaceFormValues>();

const { useStepper, utils } = defineStepper(
  {
    id: 'environment',
    label: 'Environment',
    schema: environmentSchema,
  },
  {
    id: 'hardware',
    label: 'Hardware',
    schema: hardwareSchema,
  },
  {
    id: 'compute',
    label: 'Compute Cluster',
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
            <FormLabel>Workspace Name (optional)</FormLabel>
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

function HardwareStep() {
  const { control } = useFormContext<CreateWorkspaceFormValues>();

  return (
    <div className='space-y-8'>
      <FormField
        control={control}
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
        control={control}
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
      case 'slurm-icon':
        return <SlurmIcon className='size-12' />;
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
