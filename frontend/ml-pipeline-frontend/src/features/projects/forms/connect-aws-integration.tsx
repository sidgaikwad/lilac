import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Spinner } from '@/components/ui/spinner';
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '@/components/ui/form';
import { useConnectAwsIntegration } from '@/services';
import { zodResolver } from '@hookform/resolvers/zod';
import { useForm } from 'react-hook-form';
import { toast } from '@/components/toast';
import { z } from 'zod';

const createProjectSchema = z.object({
  roleArn: z.string().nonempty(),
  placeholderExternalId: z.string().nonempty(),
});

type ConnectIntegrationFormInputs = z.infer<typeof createProjectSchema>;

export interface ConnectAwsIntegrationFormProps {
  projectId: string;
  onSubmit: () => void;
  onCancel: () => void;
}

export function ConnectAwsIntegrationForm(
  props: ConnectAwsIntegrationFormProps
) {
  const { mutate: connectIntegration, isPending } = useConnectAwsIntegration({
    onSuccess: () => {
      toast.success('S3 integration connected successfully!');
    },
    onError: (error) => toast.error(error.error),
  });

  const form = useForm<ConnectIntegrationFormInputs>({
    resolver: zodResolver(createProjectSchema),
    defaultValues: {
      roleArn: '',
      placeholderExternalId: '',
    },
  });

  const onSubmit = (data: ConnectIntegrationFormInputs) => {
    connectIntegration({
      projectId: props.projectId,
      roleArn: data.roleArn,
      placeholderExternalId: data.placeholderExternalId,
    });
    props.onSubmit();
  };

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className='space-y-4'>
        <FormField
          control={form.control}
          name='roleArn'
          render={({ field }) => (
            <FormItem>
              <FormLabel>Role ARN</FormLabel>
              <FormControl>
                <Input
                  placeholder='arn:aws:iam::123456789012:role/example-role'
                  {...field}
                />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name='placeholderExternalId'
          render={({ field }) => (
            <FormItem>
              <FormLabel>Placeholder External Id</FormLabel>
              <FormControl>
                <Input placeholder='0000' {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <Button
          className='mr-1'
          variant='outline'
          disabled={isPending}
          onClick={props.onCancel}
        >
          <span>Cancel</span>
        </Button>
        <Button type='submit' disabled={isPending}>
          {isPending ? <Spinner size='small' /> : <span>Submit</span>}
        </Button>
      </form>
    </Form>
  );
}
