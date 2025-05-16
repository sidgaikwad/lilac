import React, { useState } from 'react';
import { useParams } from 'react-router-dom';
import { ReactFlowProvider } from '@xyflow/react';
import PipelineSidebar from '../components/PipelineSidebar';
import DatasetSelectionModal from '../components/DatasetSelectionModal';
import PipelineEditorFlow from '../components/PipelineEditorFlow';
import { toast } from 'sonner';
import { Button } from '@/components/ui/button';
import { PlayIcon, SaveIcon } from 'lucide-react';
import { Skeleton } from '@/components/ui/skeleton';
import useReactFlowStore from '@/store/useReactFlowStore';
import { shallow } from 'zustand/shallow';
import { StepDefinition } from '@/types';
import { useSuspenseQuery } from '@tanstack/react-query';
import { getPipelineQuery } from '@/services';
import { UpdatePipelineRequest, useUpdatePipeline } from '@/services';
import { useRunPipeline } from '@/services';

const PipelineEditorPage: React.FC = () => {
  const { pipelineId } = useParams<{ pipelineId: string }>();
  const [isDatasetModalOpen, setIsDatasetModalOpen] = useState(false);

  const { data: pipeline } = useSuspenseQuery(getPipelineQuery(pipelineId));

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
    <ReactFlowProvider>
      <div className="flex flex-col h-full">
        <div className="p-2 border-b border-border bg-card flex justify-between">
          <h1 className="font-bold text-2xl">
            {pipeline?.name ?? <Skeleton className="h-8 w-48" />}
          </h1>
          <div className="flex justify-end space-x-4">
            <Button
              onClick={() => {
                const updateRequest: UpdatePipelineRequest = {
                  pipelineId: pipelineId!,
                  steps: nodes.map((node) => ({
                    stepId: node.id,
                    stepDefinitionId: (
                      node.data.stepDefinition as StepDefinition
                    ).id,
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
          </div>
        </div>
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
  );
};

export default PipelineEditorPage;
