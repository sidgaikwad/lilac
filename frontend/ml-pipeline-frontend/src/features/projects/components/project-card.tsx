import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardAction,
} from '@/components/ui/card';
import DeleteProjectModal from '@/features/projects/components/delete-project-modal';
import { Project } from '@/types';
import { Link } from 'react-router-dom';

export interface ProjectCardProps {
  project: Project;
}

function ProjectCard(props: ProjectCardProps) {
  return (
    <Card
      className="w-full max-w-[300px] basis-full md:basis-1/2 lg:basis-1/4"
      key={props.project.id}
    >
      <CardHeader>
        <Link
          to={`/projects/${props.project.id}`}
          className="text-primary hover:underline"
        >
          <CardTitle>{props.project.name}</CardTitle>
        </Link>
        <CardDescription>Description</CardDescription>
        <CardAction>
          <DeleteProjectModal project={props.project} />
        </CardAction>
      </CardHeader>
    </Card>
  );
}

export default ProjectCard;
