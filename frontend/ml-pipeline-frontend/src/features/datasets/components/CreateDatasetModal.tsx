import {
    Dialog,
    DialogTrigger,
    DialogContent,
    DialogHeader,
    DialogTitle,
    Button,
    Label,
    Input,
    Spinner,
    DialogFooter,
    DialogClose,
  } from '@/components/ui';
  import { useCreateDataset } from '@/services/controlplane-api';
  import { zodResolver } from '@hookform/resolvers/zod';
  import { useForm } from 'react-hook-form';
  import { toast } from 'sonner';
  import { z } from 'zod';
  
  const createDatasetSchema = z.object({
    name: z
      .string()
      .min(3, { message: 'Dataset name must be at least 3 characters' }),
    description: z
        .string()
        .optional(),
    images: z.instanceof(FileList).refine(files => files.length > 0, {
      message: 'At least one image must be selected.',
    })
  });
  
  type CreateDatasetFormInputs = z.infer<typeof createDatasetSchema>;
  
  export interface CreateDatasetModalProps {
    isOpen: boolean;
    setOpen: (isOpen: boolean) => void;
    projectId: string;
  }
  
  const CreateDatasetModal: React.FC<CreateDatasetModalProps> = (
    props: CreateDatasetModalProps
  ) => {
    const { mutate: createDataset, isPending } = useCreateDataset({
      onSuccess: (_data) => toast.success('Successfully created dataset!'),
      onError: (error) => toast.error(error.error),
    });
  
    const {
      register,
      handleSubmit,
      formState: { errors },
    } = useForm<CreateDatasetFormInputs>({
      resolver: zodResolver(createDatasetSchema),
    });

    const readImageData = (file: File): Promise<string> =>
      new Promise((resolve, reject) => {
        const reader = new FileReader()
        reader.readAsDataURL(file);
        reader.onload = () => {
          resolve(reader.result as string);
        }
        reader.onerror = (error) => reject(error)
    });
  
    
    const onSubmit = async (data: CreateDatasetFormInputs) => {
        console.log(data);
        const images = [];
        for (let i = 0; i < data.images.length; i++) {
          const image = data.images[i];
            const imageData = await readImageData(image);
            images.push({
              metadata: {
                fileName: image.name,
                fileType: image.type,
                size: image.size,
                createdAt: new Date(image.lastModified).toISOString(),
              },
              contents: imageData,
            });
        }
      createDataset({ datasetName: data.name, projectId: props.projectId, description: data.description, images });
      props.setOpen(false);
    };
  
    return (
      <Dialog open={props.isOpen} onOpenChange={props.setOpen}>
        <DialogTrigger asChild>
          <Button>Create Dataset</Button>
        </DialogTrigger>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Create Dataset</DialogTitle>
          </DialogHeader>
          <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
            <div className="flex items-center justify-between bg-background">
              <div className="bg-card text-card-foreground rounded shadow-md w-96">
                <div className="w-full flex-1 gap-2 space-y-2">
                  <Label htmlFor="name">Name</Label>
                  <Input
                    id="name"
                    type="text"
                    placeholder="Dataset name"
                    {...register('name')}
                    aria-invalid={errors.name ? 'true' : 'false'}
                    disabled={isPending}
                  />
                  {errors.name && (
                    <p className="text-sm text-destructive">
                      {errors.name.message}
                    </p>
                  )}
                </div>
                <div className="w-full flex-1 gap-2 space-y-2">
                  <Label htmlFor="description">Description</Label>
                  <Input
                    id="description"
                    type="text"
                    placeholder="Dataset description"
                    {...register('description')}
                    aria-invalid={errors.description ? 'true' : 'false'}
                    disabled={isPending}
                  />
                  {errors.description && (
                    <p className="text-sm text-destructive">
                      {errors.description.message}
                    </p>
                  )}
                </div>
                <div className="w-full flex-1 gap-2 space-y-2">
                  <Label htmlFor="images">Image Selection</Label>
                  <Input
                    id="description"
                    type="file"
                    placeholder="Select images"
                    {...register('images')}
                    aria-invalid={errors.images ? 'true' : 'false'}
                    disabled={isPending}
                    multiple
                  />
                  {errors.images && (
                    <p className="text-sm text-destructive">
                      {errors.images.message}
                    </p>
                  )}
                </div>
              </div>
            </div>
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
        </DialogContent>
      </Dialog>
    );
  };
  
  export default CreateDatasetModal;
  