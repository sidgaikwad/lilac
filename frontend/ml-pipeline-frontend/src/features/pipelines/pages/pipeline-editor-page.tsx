import { useState } from 'react';
import { useParams } from 'react-router-dom';
import { ReactFlowProvider } from '@xyflow/react';
import PipelineSidebar from '../components/pipeline-sidebar';
import DatasetSelectionModal from '../components/dataset-selection-modal';
import PipelineEditorFlow from '../components/pipeline-editor-flow';
import { toast } from 'sonner';
import { Button } from '@/components/ui/button';
import { PlayIcon, SaveIcon } from 'lucide-react';
import useReactFlowStore from '@/store/use-react-flow-store';
import { shallow } from 'zustand/shallow';
import { StepDefinition } from '@/types';
import { useSuspenseQuery } from '@tanstack/react-query';
import { getPipelineQuery, getProjectQuery } from '@/services';
import { UpdatePipelineRequest, useUpdatePipeline } from '@/services';
import { useRunPipeline } from '@/services';
import {
  Container,
  ContainerAction,
  ContainerContent,
  ContainerDescription,
  ContainerHeader,
  ContainerTitle,
} from '@/components/ui/container';
import Breadcrumbs from '@/components/common/breadcrumbs';

function PipelineEditorPage() {
  const { pipelineId, projectId } = useParams<{
    projectId: string;
    pipelineId: string;
  }>();
  const [isDatasetModalOpen, setIsDatasetModalOpen] = useState(false);

  const { data: pipeline } = useSuspenseQuery(getPipelineQuery(pipelineId));
  const { data: project } = useSuspenseQuery(getProjectQuery(projectId));

  const { mutate: updatePipeline, isPending: isSavingPipeline } =
    useUpdatePipeline({
      onSuccess: () => toast.success('Successfully saved pipeline!'),
      onError: (err) => {
        toast.error('Failed to save pipeline', {
          description: `${err.statusCode} ${err.error}`,
        });
      },
    });

  const { mutate: runPipeline } = useRunPipeline({
    onSuccess: (data) =>
      toast.success('Successfully submitted pipeline job', {
        description: `${data.id}`,
      }),
    onError: (err) => {
      toast.error('Failed to save pipeline', {
        description: `${err.statusCode} ${err.error}`,
      });
    },
  });

  const { nodes, edges } = useReactFlowStore(
    (state) => ({
      nodes: state.nodes,
      edges: state.edges,
    }),
    shallow
  );

  return (
    <Container>
      <ContainerHeader>
        <div className="flex-1 shrink-0 grow-0 basis-full pb-4">
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
                link: `/projects/${pipeline.projectId}/pipelines`,
              },
              {
                content: pipeline.name,
                link: `/projects/${pipeline.projectId}/pipelines/${pipeline.id}`,
              },
            ]}
          />
        </div>
        <ContainerTitle>
          {pipeline.name}
          <ContainerDescription></ContainerDescription>
        </ContainerTitle>
        <ContainerAction>
          <Button
            onClick={() => {
              const updateRequest: UpdatePipelineRequest = {
                pipelineId: pipelineId!,
                steps: nodes.map((node) => ({
                  stepId: node.id,
                  stepDefinitionId: (node.data.stepDefinition as StepDefinition)
                    .id,
                  stepParameters: node.data.parameters as Record<
                    string,
                    string | number | boolean | object
                  >,
                })),
                stepConnections: edges.map((edge) => [
                  edge.source,
                  edge.target,
                ]),
              };
              updatePipeline(updateRequest);
            }}
            size="sm"
            variant="outline"
            disabled={isSavingPipeline || !pipeline}
          >
            <SaveIcon className="mr-2 h-4 w-4" />{' '}
            {isSavingPipeline ? 'Saving...' : 'Save Pipeline'}
          </Button>
          <Button
            onClick={() => {
              if (pipelineId) {
                setIsDatasetModalOpen(true);
              }
            }}
            size="sm"
            disabled={false}
          >
            <PlayIcon className="mr-2 h-4 w-4" /> Run Pipeline
          </Button>
        </ContainerAction>
      </ContainerHeader>

      <ContainerContent>
        <ReactFlowProvider>
          <div className="flex h-full flex-col">
            <div className="flex flex-grow overflow-hidden">
              {}
              {pipeline && (
                <PipelineEditorFlow key={pipelineId} pipeline={pipeline} />
              )}
              <PipelineSidebar />
            </div>
            {pipelineId && (
              <DatasetSelectionModal
                isOpen={isDatasetModalOpen}
                onClose={() => setIsDatasetModalOpen(false)}
                onSelectDataset={(datasetId) => {
                  runPipeline({ pipelineId, datasetId });
                }}
              />
            )}
          </div>
        </ReactFlowProvider>
      </ContainerContent>
    </Container>
  );
};

export default PipelineEditorPage;
