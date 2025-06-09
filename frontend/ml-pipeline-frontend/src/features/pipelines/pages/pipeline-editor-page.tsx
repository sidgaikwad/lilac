import { useState } from 'react';
import { useParams } from 'react-router-dom';
import { ReactFlowProvider, Node, Edge } from '@xyflow/react';
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

interface PipelineValidationResult {
  isValid: boolean;
  errors: string[];
}

function validatePipeline(
  nodes: Node[],
  edges: Edge[]
): PipelineValidationResult {
  const errors: string[] = [];

  if (nodes.length === 0) {
    errors.push('Pipeline has no steps. Add steps to the canvas.');
  }

  // To-Do fix parameter checking

  if (nodes.length > 1 && edges.length === 0) {
    errors.push('Pipeline has multiple steps but no connections between them.');
  }

  const _nodeIds = new Set(nodes.map((node) => node.id));
  const connectedNodes = new Set<string>();
  edges.forEach((edge) => {
    connectedNodes.add(edge.source);
    connectedNodes.add(edge.target);
  });

  if (nodes.length > 0) {
    for (const node of nodes) {
      if (
        edges.length > 0 &&
        nodes.length > 1 &&
        !connectedNodes.has(node.id)
      ) {
        const stepDef = node.data.stepDefinition as StepDefinition;
        errors.push(
          `Step "${stepDef?.name || node.id}" is isolated and not connected.`
        );
      }
    }
  }

  if (nodes.length > 1) {
    const sourceNodes = nodes.filter(
      (n) => !edges.some((e) => e.target === n.id)
    );
    const sinkNodes = nodes.filter(
      (n) => !edges.some((e) => e.source === n.id)
    );

    if (sourceNodes.length === 0 && nodes.length > 0) {
      // Check nodes.length > 0 for single node case
      errors.push(
        'Pipeline has no clear starting step (a step with no inputs). This might be a cycle.'
      );
    }
    if (sinkNodes.length === 0 && nodes.length > 0) {
      // Check nodes.length > 0 for single node case
      errors.push(
        'Pipeline has no clear ending step (a step with no outputs).'
      );
    }
  }

  return {
    isValid: errors.length === 0,
    errors,
  };
}

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
      toast.error('Failed to run pipeline', {
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
            size='sm'
            variant='outline'
            disabled={isSavingPipeline || !pipeline}
          >
            <SaveIcon className='mr-2 h-4 w-4' />{' '}
            {isSavingPipeline ? 'Saving...' : 'Save Pipeline'}
          </Button>
          <Button
            onClick={() => {
              const validationResult = validatePipeline(nodes, edges);
              if (!validationResult.isValid) {
                validationResult.errors.forEach((err) =>
                  toast.error(err, { duration: 5000 })
                );
                return;
              }
              if (pipelineId) {
                setIsDatasetModalOpen(true);
              }
            }}
            size='sm'
          >
            <PlayIcon className='mr-2 h-4 w-4' /> Run Pipeline
          </Button>
        </ContainerAction>
      </ContainerHeader>

      <ContainerContent>
        <ReactFlowProvider>
          <div className='flex h-full flex-col'>
            <div className='flex flex-grow overflow-hidden'>
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
}

export default PipelineEditorPage;
