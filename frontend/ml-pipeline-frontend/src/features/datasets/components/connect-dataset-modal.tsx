import {
  Dialog,
  DialogTrigger,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
  DialogClose,
  DialogDescription,
} from '@/components/ui/dialog';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Spinner } from '@/components/ui/spinner';
import { useCreateDataset } from '@/services';
import { zodResolver } from '@hookform/resolvers/zod';
import { useForm } from 'react-hook-form';
import { toast } from 'sonner';
import { z } from 'zod';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import {
  Form,
  FormField,
  FormItem,
  FormLabel,
  FormControl,
  FormMessage,
} from '@/components/ui/form';

const createDatasetSchema = z.object({
  name: z.string().min(3),
  description: z.string().optional(),
  source: z.discriminatedUnion('sourceType', [
    z.object({
      sourceType: z.literal('S3'),
      bucketName: z.string(),
    }),
  ]),
});

type ConnectDatasetFormInputs = z.infer<typeof createDatasetSchema>;

export interface ConnectDatasetModalProps {
  isOpen: boolean;
  setOpen: (isOpen: boolean) => void;
  projectId: string;
}

const ConnectDatasetModal: React.FC<ConnectDatasetModalProps> = (
  props: ConnectDatasetModalProps
) => {
  const { mutate: createDataset, isPending } = useCreateDataset({
    onSuccess: (_data) => toast.success('Successfully conneced dataset!'),
    onError: (error) => toast.error(error.error),
  });

  const form = useForm<ConnectDatasetFormInputs>({
    resolver: zodResolver(createDatasetSchema),
    defaultValues: {
      source: {
        sourceType: 'S3',
      },
    },
  });

  const sourceType = form.watch('source.sourceType');

  const onSubmit = async (data: ConnectDatasetFormInputs) => {
    createDataset({
      datasetName: data.name,
      projectId: props.projectId,
      description: data.description,
      source: data.source,
    });
    props.setOpen(false);
  };

  return (
    <Dialog open={props.isOpen} onOpenChange={props.setOpen}>
      <DialogTrigger asChild>
        <Button>Connect Dataset</Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Connect Dataset</DialogTitle>
          <DialogDescription></DialogDescription>
        </DialogHeader>

        <Form {...form}>
          <form
            onSubmit={form.handleSubmit(onSubmit)}
            className='w-2/3 space-y-6'
          >
            <FormField
              control={form.control}
              name='name'
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Name</FormLabel>
                  <FormControl>
                    <Input placeholder='name' {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name='description'
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Description</FormLabel>
                  <FormControl>
                    <Input placeholder='description' {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name='source.sourceType'
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Source Type</FormLabel>
                  <Select
                    key={field.value}
                    onValueChange={field.onChange}
                    {...field}
                  >
                    <FormControl>
                      <SelectTrigger>
                        <SelectValue placeholder='Select the source of your data' />
                      </SelectTrigger>
                    </FormControl>
                    <SelectContent>
                      <SelectItem value='S3'>S3</SelectItem>
                    </SelectContent>
                  </Select>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name='source.bucketName'
              render={({ field }) => (
                <FormItem hidden={sourceType !== 'S3'}>
                  <FormLabel>Bucket Name</FormLabel>
                  <FormControl>
                    <Input placeholder='example-bucket' {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />

            <DialogFooter>
              <Button type='submit' disabled={isPending}>
                {isPending ? <Spinner size='small' /> : 'Submit'}
              </Button>
              <DialogClose asChild>
                <Button variant='outline'>Cancel</Button>
              </DialogClose>
            </DialogFooter>
          </form>
        </Form>
      </DialogContent>
    </Dialog>
  );
};

export default ConnectDatasetModal;
