import { Card, CardHeader, CardTitle, CardDescription } from '@/components/ui';
import useOrganizationStore from '@/store/useOrganizationStore';
import { Organization, Project } from '@/types';
import { Link } from 'react-router-dom';

export interface ProjectCardProps {
  organization: Organization;
  project: Project;
}

const ProjectCard: React.FC<ProjectCardProps> = (props: ProjectCardProps) => {
  const { setSelectedOrganizationId, setSelectedProjectId } =
    useOrganizationStore();

  return (
    <Card
      className="basis-full md:basis-1/2 lg:basis-1/4 w-full max-w-[300px]"
      key={props.project.id}
    >
      <CardHeader>
        <Link
          to={`/projects/${props.project.id}`}
          className="text-primary hover:underline"
          onClick={(_e) => {
            setSelectedOrganizationId(props.organization.id);
            setSelectedProjectId(props.project.id);
          }}
        >
          <CardTitle>{props.project.name}</CardTitle>
        </Link>
        <CardDescription>Description</CardDescription>
      </CardHeader>
      {/* <CardFooter className="border-t pt-4 flex justify-end gap-2">
                          </CardFooter> */}
    </Card>
  );
};

export default ProjectCard;
