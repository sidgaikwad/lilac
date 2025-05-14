import {
  Dialog,
  DialogTrigger,
  DialogContent,
  DialogHeader,
  DialogTitle,
  Button,
  Input,
  Spinner,
  DialogFooter,
  DialogClose,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  DialogDescription,
} from '@/components/ui';
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '@/components/ui/form';
import { useCreateProject } from '@/services/controlplane-api/useCreateProject.hook';
import { Organization } from '@/types';
import { zodResolver } from '@hookform/resolvers/zod';
import { useEffect } from 'react';
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
  setOpen: (isOpen: boolean) => void;
  organizations: Organization[];
  organizationId?: string;
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
      organizationId: props.organizationId,
    },
  });

  useEffect(() => {
    if (props.organizationId !== undefined) {
      form.setValue('organizationId', props.organizationId);
    }
  }, [props.organizationId]);

  
  const onSubmit = (data: CreateProjectFormInputs) => {
    createProject({
      name: data.projectName,
      organizationId: data.organizationId,
    });
    form.reset();
    props.setOpen(false);
  };

  return (
    <Dialog open={props.isOpen} onOpenChange={props.setOpen}>
      <DialogTrigger asChild>
        <Button
          onClick={() => {
            form.resetField('organizationId');
            props.setOpen(true);
          }}
          variant="outline"
          disabled={!props.organizations || props.organizations.length === 0}
        >
          Create Project
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Create Project</DialogTitle>
          {props.organizationId && props.organizations.find(org => org.id === props.organizationId) && (
            <DialogDescription>
              In Organization: {props.organizations.find(org => org.id === props.organizationId)?.name}
            </DialogDescription>
          )}
        </DialogHeader>
        <Form {...form}>
          <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
            {}
            <FormField
              control={form.control}
              name="projectName"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Project Name</FormLabel>
                  <FormControl>
                    <Input placeholder="my project" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <DialogFooter>
              <div className="flex items-center bg-background w-full">
                <div className="flex bg-card justify-between text-card-foreground rounded shadow-md w-96">
                  <Button type="submit" disabled={isPending}>
                    {isPending ? (
                      <Spinner size="small" />
                    ) : (
                      <span>Submit</span>
                    )}
                  </Button>
                  <DialogClose asChild>
                    <Button
                      className="mr-1"
                      variant="outline"
                      disabled={isPending}
                    >
                      <span>Cancel</span>
                    </Button>
                  </DialogClose>
                </div>
              </div>
            </DialogFooter>
          </form>
        </Form>
      </DialogContent>
    </Dialog>
  );
};

export default CreateProjectModal;
