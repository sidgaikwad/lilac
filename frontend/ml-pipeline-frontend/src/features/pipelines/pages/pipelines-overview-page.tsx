import { useState } from 'react';
import { Link, useParams } from 'react-router-dom';
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardAction,
} from '@/components/ui/card';
import { WorkflowIcon } from 'lucide-react';
import EmptyCardSection from '@/components/common/empty-card-section';
import CreatePipelineModal from '../components/create-pipeline-modal';
import { getProjectQuery, useListPipelines } from '@/services';
import {
  Container,
  ContainerAction,
  ContainerContent,
  ContainerDescription,
  ContainerHeader,
  ContainerTitle,
} from '@/components/ui/container';
import Breadcrumbs from '@/components/common/breadcrumbs';
import { useSuspenseQuery } from '@tanstack/react-query';
import { Spinner } from '@/components/ui/spinner';
import DeletePipelineModal from '../components/delete-pipeline-modal';

function PipelinesOverviewPage() {
  const { projectId } = useParams<{ projectId: string }>();

  const [isCreatePipelineModalOpen, setCreatePipelineModalOpen] =
    useState(false);

  const { data: project } = useSuspenseQuery(getProjectQuery(projectId));
  const {
    data: pipelines = [],
    isLoading: isLoadingPipelines,
    error: _pipelineError,
  } = useListPipelines({ projectId });

  return (
    <Container>
      <ContainerHeader>
        <div className='flex-1 shrink-0 grow-0 basis-full pb-4'>
          <Breadcrumbs
            breadcrumbs={[
              {
                content: 'Projects',
                link: `/organizations/${project?.organizationId}/projects`,
              },
              {
                content: project?.name ?? projectId,
                link: `/projects/${projectId}`,
              },
              {
                content: 'Pipelines',
                link: `/projects/${projectId}/pipelines`,
              },
            ]}
          />
        </div>
        <ContainerTitle>
          Pipelines
          <ContainerDescription></ContainerDescription>
        </ContainerTitle>
        <ContainerAction>
          <CreatePipelineModal
            isOpen={isCreatePipelineModalOpen}
            setOpen={setCreatePipelineModalOpen}
            projectId={projectId ?? ''}
          />
        </ContainerAction>
      </ContainerHeader>

      <ContainerContent>
        <div className='grid h-full w-full gap-4'>
          {isLoadingPipelines && <Spinner size={'large'} />}
          {pipelines.length > 0 ? (
            pipelines.map((p) => (
              <Card
                key={p.id}
                className='flex flex-col transition-shadow duration-200 ease-in-out hover:shadow-lg'
              >
                <CardHeader>
                  <Link
                    to={`/projects/${projectId}/pipelines/${p.id}`}
                    className='group'
                  >
                    <CardTitle className='group-hover:text-primary flex items-center text-lg transition-colors'>
                      <WorkflowIcon className='text-muted-foreground group-hover:text-primary mr-2 h-5 w-5 flex-shrink-0 transition-colors' />
                      <span className='truncate' title={p.name}>
                        {p.name}
                      </span>
                    </CardTitle>
                  </Link>
                  <CardDescription className='h-16 overflow-hidden pt-1 text-sm leading-relaxed text-ellipsis'>
                    {p.description || 'No description available.'}
                  </CardDescription>
                  <CardAction>
                    <DeletePipelineModal projectId={projectId!} pipeline={p} />
                  </CardAction>
                </CardHeader>
              </Card>
            ))
          ) : (
            <EmptyCardSection
              title='No Pipelines'
              description='Get started by creating your first pipeline for this project.'
              buttonText='Create Pipeline'
              onClick={() => setCreatePipelineModalOpen(true)}
            />
          )}
        </div>
      </ContainerContent>
    </Container>
  );
}

export default PipelinesOverviewPage;
