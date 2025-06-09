import { Button } from '@/components/ui/button';
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
  CardFooter,
} from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Project } from '@/types';

export interface GeneralSettingsProps {
  project: Project;
}

export function GeneralSettings(props: GeneralSettingsProps) {
  const { project } = props;

  return (
    <Card>
      <CardHeader>
        <CardTitle>General</CardTitle>
        <CardDescription>Update project details.</CardDescription>
      </CardHeader>
      <CardContent>
        <div key={project.id} className='max-w-sm space-y-2'>
          <Label htmlFor='projectName'>Name</Label>
          <Input
            id='projectName'
            defaultValue={project.name}
            disabled={!project}
          />
        </div>
      </CardContent>
      <CardFooter>
        <Button onClick={() => {}} disabled>
          Save Name
        </Button>
      </CardFooter>
    </Card>
  );
}
