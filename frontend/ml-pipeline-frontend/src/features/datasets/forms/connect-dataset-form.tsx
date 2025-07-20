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
import { S3Icon } from '@/icons/s3';
import createFormStore from '@/store/use-form-data';
import { Input } from '@/components/ui/input';
import { SnowflakeIcon } from '@/icons/snowflake';
import {
  TestDatasetResponse,
  useTestDataset,
} from '@/services/datasets/test-dataset-connection.mutation';
import { useCallback, useEffect, useState } from 'react';
import { Alert } from '@/components/common/alert';
import { RotateCcw } from 'lucide-react';

const dataSourceTypeSchema = z.object({
  datasetSource: z.object({
    sourceType: z.enum(['S3', 'Snowflake']),
  }),
});
const dataSourceSchema = z.object({
  datasetSource: z.discriminatedUnion('sourceType', [
    z.object({
      sourceType: z.literal('S3'),
      bucketName: z.string(),
      accessKey: z.string(),
      secretKey: z.string(),
    }),
    z.object({
      sourceType: z.literal('Snowflake'),
      username: z.string(),
      password: z.string(),
      account: z.string(),
      warehouse: z.string().optional(),
      database: z.string().optional(),
      schema: z.string().optional(),
      role: z.string().optional(),
    }),
  ]),
});
const dataSetSchema = z.object({
  datasetName: z.string(),
  datasetDescription: z.string(),
  ...dataSourceSchema.shape,
});

const useFormStore = createFormStore<z.infer<typeof dataSourceSchema>>();

type DataSourceTypeFormValues = z.infer<typeof dataSourceTypeSchema>;
type DataSourceFormValues = z.infer<typeof dataSourceSchema>;
export type DataSetFormValues = z.infer<typeof dataSetSchema>;

const { useStepper, steps, utils } = defineStepper(
  { id: 'selectSource', label: 'Select Source', schema: dataSourceTypeSchema },
  {
    id: 'configureDatasource',
    label: 'Configure Source',
    schema: dataSourceSchema,
  },
  {
    id: 'configureDataset',
    label: 'Configure Dataset',
    schema: dataSetSchema,
  },
  {
    id: 'testConnection',
    label: 'Test Connection',
    schema: dataSetSchema,
  }
);

export interface ConnectDatasetFormProps {
  projectId: string;
  onSubmit: (values: DataSetFormValues) => void;
}

function ConnectDatasetForm(props: ConnectDatasetFormProps) {
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
      props.onSubmit(values as DataSetFormValues);
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
        <nav aria-label='Connect Dataset Steps' className='group my-4'>
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
                        selectSource: () => <SelectDataSource />,
                        configureDatasource: () => {
                          switch (formValues.datasetSource?.sourceType) {
                            case 'S3':
                              return <ConfigureS3 />;
                            case 'Snowflake':
                              return <ConfigureSnowflake />;
                          }
                        },
                        configureDataset: () => <ConfigureDataset />,
                        testConnection: () => (
                          <TestDataset projectId={props.projectId} />
                        ),
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

function SelectDataSource() {
  const { register, control } = useFormContext<DataSourceTypeFormValues>();

  return (
    <FormField
      name={register('datasetSource.sourceType').name}
      control={control}
      render={({ field }) => {
        return (
          <FormItem className='space-y-3'>
            <FormLabel>Source Type</FormLabel>
            <FormMessage />
            <FormControl>
              <RadioGroupPrimitive.Root
                onValueChange={field.onChange}
                defaultValue={field.value}
                className='grid grid-cols-2 gap-4'
              >
                <FormItem className='flex items-center gap-3'>
                  <div>
                    <RadioGroupPrimitive.Item
                      key='S3'
                      value='S3'
                      className={cn(
                        'group relative rounded-xl text-start',
                        'data-[state=checked]:ring-accent-border-hover data-[state=checked]:ring-2'
                      )}
                    >
                      <Card
                        icon={<S3Icon className='size-24' />}
                        title='AWS S3'
                        description='Connect an S3 bucket.'
                      />
                    </RadioGroupPrimitive.Item>
                  </div>
                </FormItem>
                <FormItem className='flex items-center gap-3'>
                  <div>
                    <RadioGroupPrimitive.Item
                      key='Snowflake'
                      value='Snowflake'
                      className={cn(
                        'group relative rounded-xl text-start',
                        'data-[state=checked]:ring-accent-border-hover data-[state=checked]:ring-2'
                      )}
                    >
                      <Card
                        icon={<SnowflakeIcon className='size-24' />}
                        title='Snowflake'
                        description='Connect a Snowflake table.'
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
  );
}

function ConfigureS3() {
  const { register } = useFormContext<DataSourceFormValues>();

  return (
    <div className='space-y-4 text-start'>
      <FormField
        name={register('datasetSource.bucketName').name}
        render={({ field }) => {
          return (
            <FormItem className='space-y-3'>
              <FormLabel>Bucket Name</FormLabel>
              <FormMessage />
              <FormControl>
                <Input {...field} placeholder='bucket-name'></Input>
              </FormControl>
            </FormItem>
          );
        }}
      />
      <FormField
        name={register('datasetSource.accessKey').name}
        render={({ field }) => {
          return (
            <FormItem className='space-y-3'>
              <FormLabel>Access Key</FormLabel>
              <FormMessage />
              <FormControl>
                <Input {...field} placeholder='AKIAIOSFODNN7EXAMPLE'></Input>
              </FormControl>
            </FormItem>
          );
        }}
      />
      <FormField
        name={register('datasetSource.secretKey').name}
        render={({ field }) => {
          return (
            <FormItem className='space-y-3'>
              <FormLabel>Secret Key</FormLabel>
              <FormMessage />
              <FormControl>
                <Input
                  {...field}
                  type='password'
                  placeholder='wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY'
                ></Input>
              </FormControl>
            </FormItem>
          );
        }}
      />
    </div>
  );
}

function ConfigureSnowflake() {
  const { register, control } = useFormContext<DataSourceFormValues>();

  return (
    <div className='space-y-4 text-start'>
      <FormField
        name={register('datasetSource.username').name}
        control={control}
        render={({ field }) => {
          return (
            <FormItem className='space-y-3'>
              <FormLabel>User Name</FormLabel>
              <FormMessage />
              <FormControl>
                <Input {...field} placeholder='username'></Input>
              </FormControl>
            </FormItem>
          );
        }}
      />
      <FormField
        name={register('datasetSource.password').name}
        control={control}
        render={({ field }) => {
          return (
            <FormItem className='space-y-3'>
              <FormLabel>Password</FormLabel>
              <FormMessage />
              <FormControl>
                <Input
                  {...field}
                  type='password'
                  placeholder='password'
                ></Input>
              </FormControl>
            </FormItem>
          );
        }}
      />
      <FormField
        name={register('datasetSource.account').name}
        control={control}
        render={({ field }) => {
          return (
            <FormItem className='space-y-3'>
              <FormLabel>Account</FormLabel>
              <FormMessage />
              <FormControl>
                <Input {...field} placeholder='account'></Input>
              </FormControl>
            </FormItem>
          );
        }}
      />
      <FormField
        name={register('datasetSource.warehouse').name}
        control={control}
        render={({ field }) => {
          return (
            <FormItem className='space-y-3'>
              <FormLabel>
                Warehouse
                <span className='text-gray-text-muted text-xs font-light italic'>
                  optional
                </span>
              </FormLabel>
              <FormMessage />
              <FormControl>
                <Input {...field} placeholder='warehouse name'></Input>
              </FormControl>
            </FormItem>
          );
        }}
      />
      <FormField
        name={register('datasetSource.database').name}
        control={control}
        render={({ field }) => {
          return (
            <FormItem className='space-y-3'>
              <FormLabel>
                Database
                <span className='text-gray-text-muted text-xs font-light italic'>
                  optional
                </span>
              </FormLabel>
              <FormMessage />
              <FormControl>
                <Input {...field} placeholder='database name'></Input>
              </FormControl>
            </FormItem>
          );
        }}
      />
      <FormField
        name={register('datasetSource.schema').name}
        control={control}
        render={({ field }) => {
          return (
            <FormItem className='space-y-3'>
              <FormLabel>
                Schema
                <span className='text-gray-text-muted text-xs font-light italic'>
                  optional
                </span>
              </FormLabel>
              <FormMessage />
              <FormControl>
                <Input {...field} placeholder='schema name'></Input>
              </FormControl>
            </FormItem>
          );
        }}
      />
      <FormField
        name={register('datasetSource.role').name}
        control={control}
        render={({ field }) => {
          return (
            <FormItem className='space-y-3'>
              <FormLabel>
                Role
                <span className='text-gray-text-muted text-xs font-light italic'>
                  optional
                </span>
              </FormLabel>
              <FormMessage />
              <FormControl>
                <Input {...field} placeholder='role name'></Input>
              </FormControl>
            </FormItem>
          );
        }}
      />
    </div>
  );
}

function ConfigureDataset() {
  const { register, control } = useFormContext<DataSetFormValues>();

  return (
    <div className='space-y-4 text-start'>
      <FormField
        name={register('datasetName').name}
        control={control}
        render={({ field }) => {
          return (
            <FormItem className='space-y-3'>
              <FormLabel>Dataset Name</FormLabel>
              <FormControl>
                <Input {...field} placeholder='dataset-name'></Input>
              </FormControl>
              <FormMessage />
            </FormItem>
          );
        }}
      />
      <FormField
        name={register('datasetDescription').name}
        control={control}
        render={({ field }) => {
          return (
            <FormItem className='space-y-3'>
              <FormLabel>Description</FormLabel>
              <FormControl>
                <Input
                  {...field}
                  placeholder='Description of your dataset'
                ></Input>
              </FormControl>
              <FormDescription />
              <FormMessage />
            </FormItem>
          );
        }}
      />
    </div>
  );
}

function TestDataset(props: { projectId: string }) {
  const formValues = useFormStore((state) => state.formValues);
  const {
    mutate: testConnection,
    isError,
    isPending,
    isSuccess,
  } = useTestDataset();
  const [result, setResult] = useState<TestDatasetResponse>();
  const runTest = useCallback(() => {
    testConnection(
      { projectId: props.projectId, ...(formValues as DataSetFormValues) },
      {
        onSuccess: (data) => {
          setResult(data);
        },
      }
    );
  }, [props.projectId, formValues, testConnection]);
  useEffect(() => {
    runTest();
  }, [runTest]);
  const getAlertVariant = () => {
    const error = isError || (isSuccess && !result?.success);
    const success = isSuccess && result?.success;
    if (success) {
      return (
        <Alert
          className='w-fit'
          variant='success'
          title='Connection successful'
          description='We were successfully able to connect to your data source! Please submit the form to create your data datasetSource.'
        />
      );
    } else if (error) {
      return (
        <Alert
          variant='error'
          title='Failed to connect'
          description={<div>Error connecting to data source</div>}
          action={
            error ? (
              <Button
                className='hover:bg-red-4 mt-2 w-fit'
                variant='ghost'
                type='button'
                onClick={runTest}
              >
                <RotateCcw />
                Retry
              </Button>
            ) : undefined
          }
        />
      );
    } else if (isPending) {
      return <Alert variant='loading' title='Testing connection' />;
    }
    return undefined;
  };

  return (
    <div className='flex flex-col items-center justify-center space-y-2'>
      {getAlertVariant()}
    </div>
  );
}

export default ConnectDatasetForm;
