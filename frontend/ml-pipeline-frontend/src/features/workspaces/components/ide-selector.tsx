import {
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '@/components/ui/form';
import { RadioGroup, RadioGroupItem } from '@/components/ui/radio-group';
import { useFormContext } from 'react-hook-form';
import { CreateWorkspaceFormValues } from '../forms/CreateWorkspaceForm';

const ides = [
  { id: 'vscode', name: 'VSCode' },
  { id: 'jupyter', name: 'JupyterLab' },
  { id: 'rstudio', name: 'RStudio' },
];

export function IDESelector() {
  const { control } = useFormContext<CreateWorkspaceFormValues>();

  return (
    <FormField
      control={control}
      name='ide'
      render={({ field }) => (
        <FormItem className='space-y-3'>
          <FormLabel>Select an IDE</FormLabel>
          <FormControl>
            <RadioGroup
              onValueChange={field.onChange}
              defaultValue={field.value}
              className='grid grid-cols-3 gap-4'
            >
              {ides.map((ide) => (
                <FormItem
                  key={ide.id}
                  className='flex items-center space-y-0 space-x-3'
                >
                  <FormControl>
                    <RadioGroupItem value={ide.id} />
                  </FormControl>
                  <FormLabel className='font-normal'>{ide.name}</FormLabel>
                </FormItem>
              ))}
            </RadioGroup>
          </FormControl>
          <FormMessage />
        </FormItem>
      )}
    />
  );
}
