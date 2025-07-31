import { zodResolver } from '@hookform/resolvers/zod';
import * as React from 'react';
import { useForm, useFormContext } from 'react-hook-form';
import { z } from 'zod/v4';

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
import { Separator } from '@/components/ui/separator';
import { defineStepper } from '@stepperize/react';
import { cn } from '@/lib/utils';
import * as RadioGroupPrimitive from '@radix-ui/react-radio-group';
import { Card } from '@/components/common/card';
import createFormStore from '@/store/use-form-data';
import { Input } from '@/components/ui/input';
import { EksLogo } from '@/icons/eks';
import { useListCredentials } from '@/services';
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { Spinner } from '@/components/ui/spinner';
import { GcpLogo } from '@/icons';

const clusterSchema = z.object({
  clusterName: z.string(),
  clusterDescription: z.string().optional(),
  clusterConfig: z.object({
    clusterType: z.enum(['aws_eks', 'gcp_gke']),
  }),
});
const clusterTypeSchema = z.object({
  clusterName: z.string(),
  clusterDescription: z.string().optional(),
  clusterConfig: z.discriminatedUnion('clusterType', [
    z.object({
      clusterType: z.literal('aws_eks'),
      clusterName: z.string(),
      region: z.string(),
    }),
    z.object({
      clusterType: z.literal('gcp_gke'),
      clusterName: z.string(),
      region: z.string(),
      projectId: z.string(),
    }),
  ]),
  credentialId: z.string(),
});

const useFormStore = createFormStore<z.infer<typeof clusterTypeSchema>>();

type ClusterFormValues = z.infer<typeof clusterSchema>;
export type ClusterTypeFormValues = z.infer<typeof clusterTypeSchema>;

const { useStepper, steps, utils } = defineStepper(
  {
    id: 'configureCluster',
    label: 'Cluster Details',
    schema: clusterSchema,
  },
  {
    id: 'clusterParams',
    label: 'Cluster Parameters',
    schema: clusterTypeSchema,
  }
);

export interface ConnectClusterFormProps {
  onSubmit: (values: ClusterTypeFormValues) => void;
}

export function CreateClusterForm(props: ConnectClusterFormProps) {
  const stepper = useStepper();
  const { formValues, setFormValues } = useFormStore((state) => ({
    formValues: state.formValues,
    setFormValues: state.setFormValues,
  }));
  const form = useForm({
    mode: 'onChange',
    resolver: zodResolver(stepper.current.schema),
  });

  const onSubmit = (values: z.infer<typeof stepper.current.schema>) => {
    if (stepper.isLast) {
      props.onSubmit(values as ClusterTypeFormValues);
      stepper.reset();
    } else {
      setFormValues(values);
      stepper.next();
    }
  };

  const currentIndex = utils.getIndex(stepper.current.id);

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)}>
        <nav aria-label='Create Clusters Steps' className='group my-4'>
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
                    aria-setsize={steps.length}
                    aria-selected={stepper.current.id === step.id}
                    className='flex size-10 items-center justify-center rounded-full'
                    onClick={async () => {
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
                        configureCluster: () => <ConfigureClusterDetails />,
                        clusterParams: () => {
                          switch (formValues.clusterConfig?.clusterType) {
                            case 'aws_eks':
                              return <ConfigureAwsCluster />;
                            case 'gcp_gke':
                              return <ConfigureGcpCluster />;
                          }
                        },
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

function ConfigureClusterDetails() {
  const { register, control } = useFormContext<ClusterFormValues>();

  return (
    <div className='space-y-4 text-start'>
      <FormField
        name={register('clusterName').name}
        render={({ field }) => {
          return (
            <FormItem>
              <FormLabel>Name</FormLabel>
              <FormDescription>
                A friendly name for your cluster
              </FormDescription>
              <FormMessage />
              <FormControl>
                <Input {...field} placeholder='cluster-1'></Input>
              </FormControl>
            </FormItem>
          );
        }}
      />
      <FormField
        name={register('clusterDescription').name}
        render={({ field }) => {
          return (
            <FormItem>
              <FormLabel>
                Description
                <span className='text-gray-text-muted text-xs font-light italic'>
                  optional
                </span>
              </FormLabel>
              <FormDescription>A description of your cluster</FormDescription>
              <FormControl>
                <Input {...field}></Input>
              </FormControl>
              <FormMessage />
            </FormItem>
          );
        }}
      />
      <FormField
        name={register('clusterConfig.clusterType').name}
        control={control}
        render={({ field }) => {
          return (
            <FormItem className='space-y-3'>
              <FormLabel>Cluster Type</FormLabel>
              <FormMessage />
              <FormControl>
                <RadioGroupPrimitive.Root
                  onValueChange={field.onChange}
                  defaultValue={field.value}
                  className='grid grid-cols-3 gap-4'
                >
                  <FormItem className='flex items-center gap-3'>
                    <div>
                      <RadioGroupPrimitive.Item
                        key='aws_eks'
                        value='aws_eks'
                        className={cn(
                          'group relative rounded-xl text-start',
                          'data-[state=checked]:ring-accent-border-hover data-[state=checked]:ring-2'
                        )}
                      >
                        <Card
                          icon={<EksLogo className='size-12 rounded-sm' />}
                          title='AWS EKS'
                          description='Connect an EKS cluster.'
                        />
                      </RadioGroupPrimitive.Item>
                    </div>
                  </FormItem>
                  <FormItem className='flex items-center gap-3'>
                    <div>
                      <RadioGroupPrimitive.Item
                        key='gcp_gke'
                        value='gcp_gke'
                        className={cn(
                          'group relative rounded-xl text-start',
                          'data-[state=checked]:ring-accent-border-hover data-[state=checked]:ring-2'
                        )}
                      >
                        <Card
                          icon={<GcpLogo className='size-12 rounded-sm' />}
                          title='GCP GKE'
                          description='Connect an GKE cluster.'
                        />
                      </RadioGroupPrimitive.Item>
                    </div>
                  </FormItem>
                </RadioGroupPrimitive.Root>
              </FormControl>
            </FormItem>
          );
        }}
      />
    </div>
  );
}

function ConfigureAwsCluster() {
  const { register } = useFormContext<ClusterTypeFormValues>();
  const { data: credentials, isLoading } = useListCredentials();

  return (
    <div className='space-y-4 text-start'>
      <FormField
        name={register('clusterConfig.clusterName').name}
        render={({ field }) => {
          return (
            <FormItem>
              <FormLabel>EKS Cluster Name</FormLabel>
              <FormDescription>
                The name of your EKS cluster in AWS.
              </FormDescription>
              <FormMessage />
              <FormControl>
                <Input {...field} placeholder='eks-cluster'></Input>
              </FormControl>
            </FormItem>
          );
        }}
      />
      <FormField
        name={register('clusterConfig.region').name}
        render={({ field }) => {
          return (
            <FormItem>
              <FormLabel>AWS Region</FormLabel>
              <FormDescription>The region your cluster is in.</FormDescription>
              <FormMessage />
              <FormControl>
                <Input {...field} placeholder='us-east-1'></Input>
              </FormControl>
            </FormItem>
          );
        }}
      />
      <FormField
        name={register('credentialId').name}
        render={({ field }) => {
          return (
            <FormItem>
              <FormLabel>Credentials</FormLabel>
              <FormDescription>
                The AWS credentials to use to connect to your cluster.
              </FormDescription>
              <FormMessage />
              <FormControl>
                <Select {...field} onValueChange={field.onChange}>
                  <SelectTrigger>
                    <SelectValue placeholder='Select Credentials' />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectGroup>
                      <SelectLabel>AWS Credentials</SelectLabel>
                      {isLoading ? (
                        <Spinner />
                      ) : (
                        credentials
                          ?.filter((cred) => cred.credentialType === 'aws')
                          .map((cred) => (
                            <SelectItem value={cred.credentialId}>
                              <div className='flex flex-col'>
                                {cred.credentialName}
                              </div>
                            </SelectItem>
                          ))
                      )}
                    </SelectGroup>
                  </SelectContent>
                </Select>
              </FormControl>
            </FormItem>
          );
        }}
      />
    </div>
  );
}

function ConfigureGcpCluster() {
  const { register } = useFormContext<ClusterTypeFormValues>();
  const { data: credentials, isLoading } = useListCredentials();

  return (
    <div className='space-y-4 text-start'>
      <FormField
        name={register('clusterConfig.clusterName').name}
        render={({ field }) => {
          return (
            <FormItem>
              <FormLabel>GKE Cluster Name</FormLabel>
              <FormDescription>
                The name of your GKE cluster in GCP.
              </FormDescription>
              <FormMessage />
              <FormControl>
                <Input {...field} placeholder='eks-cluster'></Input>
              </FormControl>
            </FormItem>
          );
        }}
      />
      <FormField
        name={register('clusterConfig.region').name}
        render={({ field }) => {
          return (
            <FormItem>
              <FormLabel>GCP Region</FormLabel>
              <FormDescription>The region your cluster is in.</FormDescription>
              <FormMessage />
              <FormControl>
                <Input {...field} placeholder='us-central1'></Input>
              </FormControl>
            </FormItem>
          );
        }}
      />
      <FormField
        name={register('clusterConfig.projectId').name}
        render={({ field }) => {
          return (
            <FormItem>
              <FormLabel>Project ID</FormLabel>
              <FormDescription>
                The project ID the cluster belongs to.
              </FormDescription>
              <FormMessage />
              <FormControl>
                <Input {...field} placeholder='example-project'></Input>
              </FormControl>
            </FormItem>
          );
        }}
      />
      <FormField
        name={register('credentialId').name}
        render={({ field }) => {
          return (
            <FormItem>
              <FormLabel>Credentials</FormLabel>
              <FormDescription>
                The GCP credentials to use to connect to your cluster.
              </FormDescription>
              <FormMessage />
              <FormControl>
                <Select {...field} onValueChange={field.onChange}>
                  <SelectTrigger>
                    <SelectValue placeholder='Select Credentials' />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectGroup>
                      <SelectLabel>GCP Credentials</SelectLabel>
                      {isLoading ? (
                        <Spinner />
                      ) : (
                        credentials
                          ?.filter((cred) => cred.credentialType === 'gcp')
                          .map((cred) => (
                            <SelectItem value={cred.credentialId}>
                              <div className='flex flex-col'>
                                {cred.credentialName}
                              </div>
                            </SelectItem>
                          ))
                      )}
                    </SelectGroup>
                  </SelectContent>
                </Select>
              </FormControl>
            </FormItem>
          );
        }}
      />
    </div>
  );
}
