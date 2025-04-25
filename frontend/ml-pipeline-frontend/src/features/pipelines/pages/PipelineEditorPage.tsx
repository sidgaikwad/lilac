import React, { useState, useEffect, useCallback, useRef } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { ReactFlowProvider, ReactFlowInstance, Node, Edge, Viewport } from 'reactflow';
import { hardcodedStepDefinitions } from '@/config/stepDefinitions';
import PipelineSidebar from '../components/PipelineSidebar';
import PipelineEditorFlow from '../components/PipelineEditorFlow';
import PipelineEditorTopBar from '../components/PipelineEditorTopBar';
// Removed localStorageUtils imports
import { toast } from 'sonner';
import { Button } from '@/components/ui/button';
import { PlayIcon, Loader2Icon } from 'lucide-react';
import { cn } from '@/lib/utils';
// Removed old service import
import { PipelineDefinition, StepDefinition, ParameterDefinition, PipelineVersion } from '@/types'; // Added PipelineVersion
import { FlowData } from '@/lib/localStorageUtils'; // Keep FlowData type for now
// Import TanStack Query hooks
import { useGetPipeline } from '@/services/controlplane-api/useGetPipeline.hook';
import { useSavePipelineVersion } from '@/services/controlplane-api/useSavePipelineVersion.hook';
import { useRenamePipeline } from '@/services/controlplane-api/usePipelineMutations.hook'; // Rename hook already exists

const buttonFocusStyle = "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 dark:focus-visible:ring-offset-gray-950";

// Function to create default nodes (remains the same)
const createDefaultNodes = (stepDefs: StepDefinition[]): Node[] => {
  const inputDef = stepDefs.find((def: StepDefinition) => def.category === 'Input');
  const outputDef = stepDefs.find((def: StepDefinition) => def.category === 'Output');
  const nodes: Node[] = [];
  if (inputDef) {
    const defaultParams = inputDef.parameters.reduce((acc: Record<string, any>, param: ParameterDefinition) => { if (param.defaultValue !== undefined) acc[param.name] = param.defaultValue; return acc; }, {});
    nodes.push({ id: 'input_node', type: 'pipelineNode', position: { x: 50, y: 150 }, data: { label: inputDef.label, stepType: inputDef.type, parameters: defaultParams, stepDefinition: inputDef } });
  } else { console.warn("Default 'Input' step definition not found."); }
  if (outputDef) {
     const defaultParams = outputDef.parameters.reduce((acc: Record<string, any>, param: ParameterDefinition) => { if (param.defaultValue !== undefined) acc[param.name] = param.defaultValue; return acc; }, {});
    nodes.push({ id: 'output_node', type: 'pipelineNode', position: { x: 650, y: 150 }, data: { label: outputDef.label, stepType: outputDef.type, parameters: defaultParams, stepDefinition: outputDef } });
  } else { console.warn("Default 'Output' step definition not found."); }
  return nodes;
};

// Helper to convert API PipelineDefinition to FlowData (remains mostly the same)
const getFlowDataFromPipeline = (
    pipeline: PipelineDefinition | null | undefined,
    stepDefs: StepDefinition[]
): FlowData | null => {
    if (!pipeline) return null;
    const stepDefMap = new Map(stepDefs.map(def => [def.type, def]));
    const nodes = pipeline.steps.map(step => {
        const definition = stepDefMap.get(step.step_type);
        return {
            id: step.id, type: 'pipelineNode',
            position: step.position || { x: Math.random() * 400, y: Math.random() * 400 },
            data: { label: definition?.label || step.step_type, stepType: step.step_type, parameters: step.parameters, stepDefinition: definition }
        };
    }).filter(node => node.data.stepDefinition); // Filter out nodes if definition missing
    const edges = pipeline.connections.map(conn => ({
        id: `e-${conn.from_step_id}-${conn.to_step_id}`, source: conn.from_step_id, target: conn.to_step_id, animated: true,
    }));
    // TODO: Viewport should ideally come from API or be persisted per user/pipeline
    const viewport: Viewport = { x: 0, y: 0, zoom: 1 };
    return { nodes, edges, viewport };
};


const PipelineEditorPage: React.FC = () => {
  const { pipelineId } = useParams<{ pipelineId: string }>();
  const navigate = useNavigate();
  // Removed local state for pipelineEntry, versions, displayNodes etc.
  const reactFlowInstanceRef = useRef<ReactFlowInstance | null>(null);

  // --- TanStack Query Hooks ---
  const { data: pipelineData, isLoading: isLoadingPipeline, error: pipelineError, isError } = useGetPipeline({ pipelineId });
  const saveMutation = useSavePipelineVersion();
  const renameMutation = useRenamePipeline();

  // TODO: Fetch step definitions via query hook
  const stepDefinitions = hardcodedStepDefinitions;

  // --- Derived State ---
  // Derive flow data directly from the query result
  const currentFlowData = React.useMemo(
      () => getFlowDataFromPipeline(pipelineData, stepDefinitions),
      [pipelineData, stepDefinitions]
  );

  // Handle navigation/toast on error
  useEffect(() => {
      if (isError && !isLoadingPipeline) {
          toast.error(`Failed to load pipeline: ${pipelineError?.message || 'Unknown error'}`);
          // Navigate back to project pipelines list (assuming projectId is in pipelineData or context)
          // For now, navigate to a general fallback
          navigate('/pipelines'); // Adjust this fallback route if needed
      }
  }, [isError, isLoadingPipeline, pipelineError, navigate]);


  // --- Event Handlers ---
  const handleSave = useCallback(() => {
    if (!pipelineId || !reactFlowInstanceRef.current || !pipelineData) {
        toast.error("Cannot save: Pipeline data or editor instance not available.");
        return;
    }
    const currentFlow: FlowData = {
      nodes: reactFlowInstanceRef.current.getNodes(),
      edges: reactFlowInstanceRef.current.getEdges(),
      viewport: reactFlowInstanceRef.current.getViewport(),
    };
    saveMutation.mutate({
        pipelineId: pipelineId,
        flowData: currentFlow,
        projectId: pipelineData.project_id // Pass projectId for invalidation context
    });
  }, [pipelineId, pipelineData, saveMutation, reactFlowInstanceRef]);

  const handleRename = useCallback((id: string, newName: string): void => {
      // Rename mutation expects { pipelineId, newName }
      renameMutation.mutate({ pipelineId: id, newName: newName }, {
          // context: { projectId: pipelineData?.project_id } // Context not needed anymore
      });
  }, [renameMutation]); // Removed pipelineData dependency

  // Removed handleSelectVersion - versioning handled by API/Query invalidation now

  const handleFlowInit = useCallback((instance: ReactFlowInstance) => {
    reactFlowInstanceRef.current = instance;
  }, []);

  const handleRun = useCallback(() => {
    // Validation logic remains the same
    if (!reactFlowInstanceRef.current || !pipelineId) return;
    const nodes = reactFlowInstanceRef.current.getNodes();
    const edges = reactFlowInstanceRef.current.getEdges();
    const edgeSources = new Set(edges.map(e => e.source));
    const edgeTargets = new Set(edges.map(e => e.target));
    let isValid = true;
    let errorMessage = "";
    const inputNodes = nodes.filter(n => n.data?.stepDefinition?.category === 'Input');
    const outputNodes = nodes.filter(n => n.data?.stepDefinition?.category === 'Output');
    const processingNodes = nodes.filter(n => n.data?.stepDefinition?.category !== 'Input' && n.data?.stepDefinition?.category !== 'Output');
    if (inputNodes.length === 0) { isValid = false; errorMessage = "Pipeline must have at least one Input node."; }
    else if (outputNodes.length === 0) { isValid = false; errorMessage = "Pipeline must have at least one Output node."; }
    else {
      for (const node of processingNodes) { if (!edgeTargets.has(node.id) || !edgeSources.has(node.id)) { isValid = false; errorMessage = `Node '${node.data.label}' is not fully connected.`; break; } }
      if (isValid) { for (const node of inputNodes) { if (node.data?.stepDefinition?.category !== 'Output' && !edgeSources.has(node.id)) { isValid = false; errorMessage = `Input node '${node.data.label}' must have an outgoing connection.`; break; } } }
      if (isValid) { for (const node of outputNodes) { if (node.data?.stepDefinition?.category !== 'Input' && !edgeTargets.has(node.id)) { isValid = false; errorMessage = `Output node '${node.data.label}' must have an incoming connection.`; break; } } }
    }
    if (!isValid) { toast.error("Invalid Pipeline", { description: errorMessage }); return; }
    console.log("Simulating pipeline run with:", { nodes, edges });
    toast.info("Pipeline validation passed! Simulating run... (check console)");
    // TODO: API Call - POST /pipeline/{pipelineId}/run
  }, [pipelineId]);


  // --- Render ---
  if (isLoadingPipeline || !currentFlowData) { // Show loading until initial data/flow is ready
    return (
        <div className="flex h-full items-center justify-center p-6">
            <Loader2Icon className="mr-2 h-6 w-6 animate-spin" /> Loading pipeline...
        </div>
    );
  }

  // Note: Version handling in TopBar needs adjustment as we only fetch latest detail now
  // The mock API doesn't support fetching specific versions yet.
  // We pass an empty array for versions for now.

  return (
    <ReactFlowProvider>
      <div className="flex flex-col h-full">
        <PipelineEditorTopBar
          pipelineId={pipelineId}
          pipelineName={pipelineData?.name || 'Loading...'} // Use name from query data
          onSave={handleSave}
          onRename={handleRename}
          versions={[]} // Pass empty array for versions for now
          selectedVersionId={pipelineData?.version_id} // Show current version ID if available
          // onSelectVersion={handleSelectVersion} // Removed version selection
          isSaving={saveMutation.isPending} // Pass saving state
        />
         <div className="p-2 border-b border-border bg-card flex justify-end">
             <Button onClick={handleRun} size="sm" className={cn(buttonFocusStyle)} disabled={saveMutation.isPending}>
                 <PlayIcon className="mr-2 h-4 w-4" /> Run Pipeline (Simulated)
             </Button>
         </div>
        <div className="flex flex-grow overflow-hidden">
          {/* Use key derived from pipelineId and versionId to force remount on data change */}
          <PipelineEditorFlow
            key={`${pipelineId}-${pipelineData?.version_id || 'initial'}`}
            initialNodes={currentFlowData.nodes}
            initialEdges={currentFlowData.edges}
            initialViewport={currentFlowData.viewport}
            onFlowInit={handleFlowInit}
          />
          <PipelineSidebar />
        </div>
      </div>
    </ReactFlowProvider>
  );
};

export default PipelineEditorPage;