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
import { AwsLogo } from '@/icons/aws';

const credentialSchema = z.object({
  credentialName: z.string(),
  credentialDescription: z.string().optional(),
  credentials: z.object({
    credentialType: z.enum(['aws']),
  }),
});
const credentialTypeSchema = z.object({
  credentialName: z.string(),
  credentialDescription: z.string().optional(),
  credentials: z.discriminatedUnion('credentialType', [
    z.object({
      credentialType: z.literal('aws'),
      accessKey: z.string(),
      secretKey: z.string(),
    }),
  ]),
});

const useFormStore = createFormStore<z.infer<typeof credentialTypeSchema>>();

type CredentialFormValues = z.infer<typeof credentialSchema>;
export type CredentialTypeFormValues = z.infer<typeof credentialTypeSchema>;

const { useStepper, steps, utils } = defineStepper(
  {
    id: 'configureCredential',
    label: 'Configure Credentials',
    schema: credentialSchema,
  },
  {
    id: 'provideSecrets',
    label: 'Provide Credentials',
    schema: credentialTypeSchema,
  }
);

export interface ConnectCredentialFormProps {
  onSubmit: (values: CredentialTypeFormValues) => void;
}

export function CreateCredentialForm(props: ConnectCredentialFormProps) {
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
      props.onSubmit(values as CredentialTypeFormValues);
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
        <nav aria-label='Create Credentials Steps' className='group my-4'>
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
                        configureCredential: () => <ConfigureCredential />,
                        provideSecrets: () => {
                          switch (formValues.credentials?.credentialType) {
                            case 'aws':
                              return <ProvideAwsCredentials />;
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

function ConfigureCredential() {
  const { register, control } = useFormContext<CredentialFormValues>();

  return (
    <div className='space-y-4 text-start'>
      <FormField
        name={register('credentialName').name}
        render={({ field }) => {
          return (
            <FormItem>
              <FormLabel>Name</FormLabel>
              <FormDescription>
                A friendly name for your credentials
              </FormDescription>
              <FormMessage />
              <FormControl>
                <Input {...field} placeholder='credentials-1'></Input>
              </FormControl>
            </FormItem>
          );
        }}
      />
      <FormField
        name={register('credentialDescription').name}
        render={({ field }) => {
          return (
            <FormItem>
              <FormLabel>
                Description
                <span className='text-gray-text-muted text-xs font-light italic'>
                  optional
                </span>
              </FormLabel>
              <FormDescription>
                A description of your credentials
              </FormDescription>
              <FormControl>
                <Input {...field}></Input>
              </FormControl>
              <FormMessage />
            </FormItem>
          );
        }}
      />
      <FormField
        name={register('credentials.credentialType').name}
        control={control}
        render={({ field }) => {
          return (
            <FormItem>
              <FormLabel>Credential Type</FormLabel>
              <FormDescription>The type of credentials to add</FormDescription>
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
                        key='aws'
                        value='aws'
                        className={cn(
                          'group relative rounded-xl text-start',
                          'data-[state=checked]:ring-accent-border-hover data-[state=checked]:ring-2'
                        )}
                      >
                        <Card
                          icon={<AwsLogo className='size-12' />}
                          title='AWS'
                          description='Configure AWS credentials.'
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

function ProvideAwsCredentials() {
  const { register } = useFormContext<CredentialTypeFormValues>();

  return (
    <div className='space-y-4 text-start'>
      <FormField
        name={register('credentials.accessKey').name}
        render={({ field }) => {
          return (
            <FormItem>
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
        name={register('credentials.secretKey').name}
        render={({ field }) => {
          return (
            <FormItem>
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
