import React, { useEffect, useState } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { ReactFlowProvider } from '@xyflow/react';
import PipelineSidebar from '../components/PipelineSidebar';
import DatasetSelectionModal from '../components/DatasetSelectionModal';
import PipelineEditorFlow from '../components/PipelineEditorFlow';
import { toast } from 'sonner';
import { Button } from '@/components/ui/button';
import { PlayIcon, SaveIcon } from 'lucide-react';
import { useGetPipeline } from '@/services/controlplane-api/useGetPipeline.hook';
import { Skeleton } from '@/components/ui/skeleton';
import { useRunPipeline, useUpdatePipeline } from '@/services/controlplane-api';
import useReactFlowStore from '@/store/useReactFlowStore';
import { shallow } from 'zustand/shallow';
import { UpdatePipelineRequest } from '@/services/controlplane-api/types';
import { StepDefinition } from '@/types';

const PipelineEditorPage: React.FC = () => {
  const { pipelineId } = useParams<{ pipelineId: string }>();
  const navigate = useNavigate();
  const [isDatasetModalOpen, setIsDatasetModalOpen] = useState(false);

  const {
    data: pipeline,
    isLoading: isLoadingPipeline,
    error: pipelineError,
    isError,
  } = useGetPipeline({ pipelineId });

const { mutate: updatePipeline, isPending: isSavingPipeline } = useUpdatePipeline({
  onSuccess: () => toast.success('Successfully saved pipeline!'),
  onError: (err) => {
    toast.error(`Failed to save pipeline: ${err.statusCode} ${err.error}`);
    },
  });

  const { mutate: runPipeline } = useRunPipeline({
    onSuccess: (data) =>
      toast.success(`Successfully submitted pipeline job: ${data.jobId}`),
    onError: (err) => {
      toast.error(`Failed to save pipeline: ${err.statusCode} ${err.error}`);
    },
  });

  const { nodes, edges } = useReactFlowStore(
    (state) => ({
      nodes: state.nodes,
      edges: state.edges,
    }),
    shallow
  );

  useEffect(() => {
    if (isError && !isLoadingPipeline) {
      toast.error(
        `Failed to load pipeline: ${pipelineError?.message || 'Unknown error'}`
      );
    }
  }, [isError, isLoadingPipeline, pipelineError, navigate]);

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
              disabled={isSavingPipeline || isLoadingPipeline || !pipeline}
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
