import React, { useState } from 'react';
import { Link, useParams } from 'react-router-dom';
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
} from '@/components/ui/card';
import { Loader2Icon, WorkflowIcon } from 'lucide-react';
import EmptyCardSection from '@/components/common/EmptyCardSection';
import CreatePipelineModal from '../components/CreatePipelineModal';
import { useListPipelines } from '@/services';

const PipelinesOverviewPage: React.FC = () => {
  const { projectId } = useParams<{ projectId: string }>();

  const [isCreatePipelineModalOpen, setCreatePipelineModalOpen] =
    useState(false);

  const {
    data: pipelines = [],
    isLoading: isLoadingPipelines,
    isFetching: isFetchingPipelines,
    error: _pipelineError,
  } = useListPipelines({ projectId });

  return (
    <div className="container space-y-6 w-full">
      <div className="flex justify-between items-center">
        <h1 className="text-3xl font-bold">Pipelines</h1>
        <div className="space-x-4">
          <CreatePipelineModal
            isOpen={isCreatePipelineModalOpen}
            setOpen={setCreatePipelineModalOpen}
            projectId={projectId ?? ''}
          />
        </div>
        {isFetchingPipelines && !isLoadingPipelines && (
          <Loader2Icon className="h-5 w-5 animate-spin text-muted-foreground ml-2" />
        )}
      </div>
      <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
        {pipelines.length > 0 ? (
          pipelines.map((p) => (
            <Card
              key={p.id}
              className="flex flex-col hover:shadow-lg transition-shadow duration-200 ease-in-out"
            >
              <CardHeader className="flex-grow pb-2">
                <Link
                  to={`/projects/${projectId}/pipelines/${p.id}`}
                  className="group"
                >
                  <CardTitle className="text-lg flex items-center group-hover:text-primary transition-colors">
                    <WorkflowIcon className="mr-2 h-5 w-5 text-muted-foreground group-hover:text-primary transition-colors flex-shrink-0" />
                    <span className="truncate" title={p.name}>
                      {p.name}
                    </span>
                  </CardTitle>
                </Link>
                <CardDescription className="pt-1 text-sm leading-relaxed h-16 overflow-hidden text-ellipsis">
                  {p.description || 'No description available.'}
                </CardDescription>
              </CardHeader>
              {}
            </Card>
          ))
        ) : (
          <EmptyCardSection
            title="No Pipelines"
            description="Get started by creating your first pipeline for this project."
            buttonText="Create Pipeline"
            onClick={() => setCreatePipelineModalOpen(true)}
          />
        )}
      </div>
    </div>
  );
};

export default PipelinesOverviewPage;
