import {
  Dialog,
  DialogTrigger,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
  DialogClose,
} from '@/components/ui/dialog';
import { Button } from '@/components/ui/button';
import { Label } from '@/components/ui/label';
import { Input } from '@/components/ui/input';
import { Spinner } from '@/components/ui/spinner';
import { useCreatePipeline } from '@/services';
import { zodResolver } from '@hookform/resolvers/zod';
import { useForm } from 'react-hook-form';
import { toast } from 'sonner';
import { z } from 'zod';

const createOrgSchema = z.object({
  pipelineName: z
    .string()
    .min(3, { message: 'Pipeline name must be at least 3 characters' }),
});

type CreateOrgFormInputs = z.infer<typeof createOrgSchema>;

export interface CreatePipelineModalProps {
  isOpen: boolean;
  setOpen: (isOpen: boolean) => void;
  projectId: string;
}

const CreatePipelineModal: React.FC<CreatePipelineModalProps> = (
  props: CreatePipelineModalProps
) => {
  const { mutate: createPipeline, isPending } = useCreatePipeline({
    onSuccess: (_data) => toast.success('Successfully created pipeline!'),
    onError: (error) => toast.error(error.error),
  });

  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<CreateOrgFormInputs>({
    resolver: zodResolver(createOrgSchema),
  });

  const onSubmit = (data: CreateOrgFormInputs) => {
    createPipeline({ name: data.pipelineName, projectId: props.projectId });
    props.setOpen(false);
  };

  return (
    <Dialog open={props.isOpen} onOpenChange={props.setOpen}>
      <DialogTrigger asChild>
        <Button>Create Pipeline</Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Create Pipeline</DialogTitle>
        </DialogHeader>
        <form onSubmit={handleSubmit(onSubmit)} className='space-y-4'>
          <div className='flex items-center justify-between'>
            <div className='text-card-foreground w-[80%] space-y-4'>
              <div className='w-full flex-1 space-y-2'>
                <Label htmlFor='pipelineName'>Name</Label>
                <Input
                  id='pipelineName'
                  type='text'
                  placeholder='Pipeline name'
                  {...register('pipelineName')}
                  aria-invalid={errors.pipelineName ? 'true' : 'false'}
                  disabled={isPending}
                />
                {errors.pipelineName && (
                  <p className='text-destructive text-sm'>
                    {errors.pipelineName.message}
                  </p>
                )}
              </div>
            </div>
          </div>
          <DialogFooter>
            <div className='flex w-full items-center'>
              <div className='text-card-foreground flex w-[80%] justify-between'>
                <Button type='submit' disabled={isPending}>
                  {isPending ? <Spinner size='small' /> : <span>Submit</span>}
                </Button>
                <DialogClose asChild>
                  <Button
                    className='mr-1'
                    variant='outline'
                    disabled={isPending}
                  >
                    <span>Cancel</span>
                  </Button>
                </DialogClose>
              </div>
            </div>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
};

export default CreatePipelineModal;
