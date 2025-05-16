import React, { useEffect } from 'react';
import useOrganizationStore from '@/store/useOrganizationStore';
import { useParams } from 'react-router-dom';
import { useGetProject } from '@/services';

type ProjectParams = {
  projectId: string;
};

const ProjectOverviewPage: React.FC = () => {
  const { projectId } = useParams<ProjectParams>();
  const { data: project } = useGetProject({ projectId });
  const { setSelectedProjectId, setSelectedOrganizationId } =
    useOrganizationStore();

  useEffect(() => {
    setSelectedOrganizationId(project?.organizationId);
    setSelectedProjectId(project?.id);
  }, [
    setSelectedOrganizationId,
    setSelectedProjectId,
    project?.id,
    project?.organizationId,
  ]);

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <h1 className="text-3xl font-bold">{project?.name}</h1>
      </div>
    </div>
  );
};
export default ProjectOverviewPage;
