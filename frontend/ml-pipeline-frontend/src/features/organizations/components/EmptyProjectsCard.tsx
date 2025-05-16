import { Button } from '@/components/ui/button';
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
} from '@/components/ui/card';

interface EmptyProjectsCardProps {
  onClick: () => void;
}

const EmptyProjectsCard: React.FC<EmptyProjectsCardProps> = (
  props: EmptyProjectsCardProps
) => {
  return (
    <Card
      className="basis-full flex-1 w-full border-dashed border-input text-center"
      key={'empty-project'}
    >
      <CardHeader>
        <CardTitle>No projects</CardTitle>
        <CardDescription>Create a project to get started.</CardDescription>
      </CardHeader>
      <CardContent>
        <Button onClick={props.onClick} variant="outline">
          Create Project
        </Button>
      </CardContent>
    </Card>
  );
};

export default EmptyProjectsCard;
