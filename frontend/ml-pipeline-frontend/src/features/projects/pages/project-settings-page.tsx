import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import { getProjectQuery } from '@/services';
import { useParams } from 'react-router-dom';
import {
  Container,
  ContainerContent,
  ContainerHeader,
  ContainerTitle,
} from '@/components/ui/container';
import { useSuspenseQuery } from '@tanstack/react-query';

function ProjectSettingsPage() {
  const { projectId } = useParams<'projectId'>();
  const { data: project } = useSuspenseQuery(getProjectQuery(projectId));

  const handleRenameProject = () => {
    console.log('Renaming org...');
  };

  return (
    <Container>
      <ContainerHeader>
        <ContainerTitle>Project Settings</ContainerTitle>
      </ContainerHeader>
      <ContainerContent>
        <Card>
          <CardHeader>
            <CardTitle>General</CardTitle>
            <CardDescription>Update project details.</CardDescription>
          </CardHeader>
          <CardContent>
            <div key={project.id} className="max-w-sm space-y-2">
              <Label htmlFor="projectName">Name</Label>
              <Input
                id="projectName"
                defaultValue={project.name}
                disabled={!project}
              />
            </div>
          </CardContent>
          <CardFooter>
            <Button onClick={handleRenameProject} disabled>
              Save Name
            </Button>
          </CardFooter>
        </Card>
      </ContainerContent>
    </Container>
  );
}

export default ProjectSettingsPage;
