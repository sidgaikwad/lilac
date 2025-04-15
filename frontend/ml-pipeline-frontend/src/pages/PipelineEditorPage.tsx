// src/pages/PipelineEditorPage.tsx (Reverted)
import React from 'react';
import { ReactFlowProvider } from 'reactflow';

// Import the layout components
import TopBar from '@/components/TopBar';
import RightSidebar from '@/components/RightSidebar';
import PipelineEditorFlow from '@/components/PipelineEditorFlow';

// No dialog props needed
const PipelineEditorPage: React.FC = () => {
  return (
    // Main container using flex-col for vertical stacking
    <div className="flex flex-col h-screen w-screen overflow-hidden">
      {/* Top navigation bar */}
      <TopBar />

      {/* Container for the main content area (editor + sidebar) */}
      <div className="flex flex-grow h-full overflow-hidden">
          {/* ReactFlowProvider wraps the components that use React Flow hooks or state */}
          <ReactFlowProvider>
            {/* Render PipelineEditorFlow without passing dialog props */}
            <PipelineEditorFlow />
            {/* Right sidebar with fixed width */}
            <RightSidebar />
          </ReactFlowProvider>
      </div>
    </div>
  );
};

export default PipelineEditorPage;
