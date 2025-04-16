import React from 'react';
import { useParams } from 'react-router-dom';
import { ReactFlowProvider } from 'reactflow';
import { hardcodedStepDefinitions } from '@/config/stepDefinitions';
import PipelineSidebar from '../components/PipelineSidebar';
import PipelineEditorFlow from '../components/PipelineEditorFlow';
import PipelineEditorTopBar from '../components/PipelineEditorTopBar';

const PipelineEditorPage: React.FC = () => {
  const { pipelineId } = useParams<{ pipelineId: string }>();
  // TODO: Fetch pipeline definition based on pipelineId using TanStack Query/SWR
  // TODO: Implement actual save functionality (API call)

  // Using hardcoded steps for now
  const availableSteps = hardcodedStepDefinitions;
  const pipelineName = `Pipeline ${pipelineId || 'New'}`; // Example name, fetch real name later

  const handleSave = () => {
    console.log("Save action triggered for pipeline:", pipelineId);
    // TODO: Gather nodes/edges state and send to backend API
    // Consider using React Flow's toObject() method
  };

  return (
    <ReactFlowProvider>
      <div className="flex flex-col h-full">
        <PipelineEditorTopBar
          pipelineName={pipelineName}
          onSave={handleSave}
        />
        <div className="flex flex-grow overflow-hidden">
          <PipelineEditorFlow />
          <PipelineSidebar availableSteps={availableSteps} />
        </div>
      </div>
    </ReactFlowProvider>
  );
};

export default PipelineEditorPage;