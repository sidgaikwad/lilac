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
import { useCreateProject } from '@/services';
import { Organization } from '@/types';
import { zodResolver } from '@hookform/resolvers/zod';
import { useForm } from 'react-hook-form';
import { toast } from 'sonner';
import { z } from 'zod';

const createProjectSchema = z.object({
  organizationId: z.string().nonempty(),
  projectName: z
    .string()
    .min(3, { message: 'Project name must be at least 3 characters' }),
});

type CreateProjectFormInputs = z.infer<typeof createProjectSchema>;

export interface CreateProjectModalProps {
  isOpen: boolean;
  setOpen: (input: boolean) => void;
  organization: Organization;
}

const CreateProjectModal: React.FC<CreateProjectModalProps> = (
  props: CreateProjectModalProps
) => {
  const { mutate: createProject, isPending } = useCreateProject({
    onSuccess: (_data) => toast.success('Successfully created project!'),
    onError: (error) => toast.error(error.error),
  });

  const form = useForm<CreateProjectFormInputs>({
    resolver: zodResolver(createProjectSchema),
    defaultValues: {
      organizationId: props.organization.id,
    },
  });

  const onSubmit = (data: CreateProjectFormInputs) => {
    createProject({
      name: data.projectName,
      organizationId: data.organizationId,
    });
    form.reset();
  };

  return (
    <Dialog open={props.isOpen} onOpenChange={props.setOpen}>
      <DialogTrigger asChild>
        <Button variant="default">Create Project</Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Create Project</DialogTitle>
        </DialogHeader>
        <Form {...form}>
          <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
            <FormItem>
              <FormLabel>Selected Organization</FormLabel>
              <FormControl>
                <Input value={props.organization.name} disabled />
              </FormControl>
              <FormMessage />
            </FormItem>
            <FormField
              control={form.control}
              name="projectName"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Project Name</FormLabel>
                  <FormControl>
                    <Input placeholder="My project" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <DialogFooter>
              <DialogClose asChild>
                <Button className="mr-1" variant="outline" disabled={isPending}>
                  <span>Cancel</span>
                </Button>
              </DialogClose>
              <Button type="submit" disabled={isPending}>
                {isPending ? <Spinner size="small" /> : <span>Submit</span>}
              </Button>
            </DialogFooter>
          </form>
        </Form>
      </DialogContent>
    </Dialog>
  );
};

export default CreateProjectModal;
