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
import { useCreateOrganization } from '@/services';
import { zodResolver } from '@hookform/resolvers/zod';
import { useForm } from 'react-hook-form';
import { toast } from 'sonner';
import { z } from 'zod';

const createOrgSchema = z.object({
  orgName: z
    .string()
    .min(3, { message: 'Organization name must be at least 3 characters' }),
});

type CreateOrgFormInputs = z.infer<typeof createOrgSchema>;

export interface CreateOrganizationModalProps {
  isOpen: boolean;
  setOpen: (isOpen: boolean) => void;
}

const CreateOrganizationModal: React.FC<CreateOrganizationModalProps> = (
  props: CreateOrganizationModalProps
) => {
  const { mutate: createOrg, isPending } = useCreateOrganization({
    onSuccess: (_data) => toast.success('Successfully created organization!'),
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
    createOrg({ name: data.orgName });
    props.setOpen(false);
  };

  return (
    <Dialog open={props.isOpen} onOpenChange={props.setOpen}>
      <DialogTrigger asChild>
        <Button>Create Organization</Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Create Organization</DialogTitle>
        </DialogHeader>
        <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
          <div className="flex items-center justify-between bg-background">
            <div className="bg-card text-card-foreground rounded shadow-md w-96">
              <div className="w-full flex-1 gap-2 space-y-2">
                <Label htmlFor="orgName">Name</Label>
                <Input
                  id="orgName"
                  type="text"
                  placeholder="Organization name"
                  {...register('orgName')}
                  aria-invalid={errors.orgName ? 'true' : 'false'}
                  disabled={isPending}
                />
                {errors.orgName && (
                  <p className="text-sm text-destructive">
                    {errors.orgName.message}
                  </p>
                )}
              </div>
            </div>
          </div>
          <DialogFooter>
            <div className="flex items-center bg-background w-full">
              <div className="flex bg-card justify-between text-card-foreground rounded shadow-md w-96">
                <Button type="submit" disabled={isPending}>
                  {isPending ? <Spinner size="small" /> : <span>Submit</span>}
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
      </DialogContent>
    </Dialog>
  );
};

export default CreateOrganizationModal;
