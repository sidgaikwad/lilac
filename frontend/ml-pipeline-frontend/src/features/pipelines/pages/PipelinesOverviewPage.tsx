import React, { useState } from 'react';
import { Link, useParams } from 'react-router-dom';
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardFooter,
} from '@/components/ui/card';
import { Loader2Icon, WorkflowIcon } from 'lucide-react';
import { useListPipelines } from '@/services/controlplane-api/useListPipelines.hook';
import EmptyCardSection from '@/components/common/EmptyCardSection';
import CreatePipelineModal from '../components/CreatePipelineModal';

const PipelinesOverviewPage: React.FC = () => {
  const { projectId } = useParams<{ projectId: string }>(); // Get projectId from URL

  const [isCreatePipelineModalOpen, setCreatePipelineModalOpen] =
    useState(false);

  // --- TanStack Query Hooks ---
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
      <div className="container flex flex-wrap gap-2 md:gap-4">
        {pipelines.length > 0 ? (
          pipelines.map((p) => (
            <Card key={p.id} className="w-full flex flex-col">
              <CardHeader className="flex-grow">
                <Link
                  to={`/projects/${projectId}/pipelines/${p.id}`}
                  className="text-primary hover:underline group"
                >
                  <CardTitle className="text-lg flex items-center">
                    <WorkflowIcon className="mr-2 h-5 w-5 text-muted-foreground group-hover:text-primary transition-colors" />
                    {p.name}
                  </CardTitle>
                </Link>
                <CardDescription>{p.description}</CardDescription>
              </CardHeader>
              <CardFooter className="border-t pt-4 flex justify-end gap-2"></CardFooter>
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
