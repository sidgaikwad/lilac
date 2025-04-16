import React, { useState, useEffect, useCallback, useRef } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { ReactFlowProvider, ReactFlowInstance, Node, Edge, Viewport } from 'reactflow';
import { hardcodedStepDefinitions } from '@/config/stepDefinitions'; // Re-add import
import PipelineSidebar from '../components/PipelineSidebar';
import PipelineEditorFlow from '../components/PipelineEditorFlow';
import PipelineEditorTopBar from '../components/PipelineEditorTopBar';
import { getPipelineEntry, addPipelineVersion, renamePipeline, PipelineVersion, FlowData, PipelineStorageEntry } from '@/lib/localStorageUtils';
import { toast } from 'sonner';
import { Button } from '@/components/ui/button';
import { PlayIcon, Loader2Icon } from 'lucide-react';
import { cn } from '@/lib/utils';
import { useQuery, useQueryClient } from '@tanstack/react-query';
import { fetchPipelineDetail } from '../services/pipelineService';
import { PipelineDefinition, StepDefinition, ParameterDefinition } from '@/types'; // Import types

const buttonFocusStyle = "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 dark:focus-visible:ring-offset-gray-950";

// Function to create default nodes (needs access to step definitions)
const createDefaultNodes = (stepDefs: StepDefinition[]): Node[] => {
  const inputDef = stepDefs.find((def: StepDefinition) => def.category === 'Input');
  const outputDef = stepDefs.find((def: StepDefinition) => def.category === 'Output');
  const nodes: Node[] = [];

  if (inputDef) {
    const defaultParams = inputDef.parameters.reduce((acc: Record<string, any>, param: ParameterDefinition) => {
        if (param.defaultValue !== undefined) acc[param.name] = param.defaultValue;
        return acc;
      }, {});
    nodes.push({ id: 'input_node', type: 'pipelineNode', position: { x: 50, y: 150 }, data: { label: inputDef.label, stepType: inputDef.type, parameters: defaultParams, stepDefinition: inputDef } });
  } else { console.warn("Default 'Input' step definition not found."); }

  if (outputDef) {
     const defaultParams = outputDef.parameters.reduce((acc: Record<string, any>, param: ParameterDefinition) => {
        if (param.defaultValue !== undefined) acc[param.name] = param.defaultValue;
        return acc;
      }, {});
    nodes.push({ id: 'output_node', type: 'pipelineNode', position: { x: 650, y: 150 }, data: { label: outputDef.label, stepType: outputDef.type, parameters: defaultParams, stepDefinition: outputDef } });
  } else { console.warn("Default 'Output' step definition not found."); }
  return nodes;
};

// Helper to convert API PipelineDefinition to FlowData
const getFlowDataFromPipeline = (
    pipeline: PipelineDefinition | null | undefined,
    stepDefs: StepDefinition[] // Need step definitions to map labels/data correctly
): FlowData | null => {
    if (!pipeline) return null;

    const stepDefMap = new Map(stepDefs.map(def => [def.type, def]));

    const nodes = pipeline.steps.map(step => {
        const definition = stepDefMap.get(step.step_type);
        return {
            id: step.id,
            type: 'pipelineNode',
            position: step.position || { x: Math.random() * 400, y: Math.random() * 400 },
            data: {
                label: definition?.label || step.step_type, // Use label from definition
                stepType: step.step_type,
                parameters: step.parameters,
                stepDefinition: definition // Pass the full definition
            }
        };
    }).filter(node => node.data.stepDefinition); // Filter out nodes whose definition wasn't found

    const edges = pipeline.connections.map(conn => ({
        id: `e-${conn.from_step_id}-${conn.to_step_id}`,
        source: conn.from_step_id,
        target: conn.to_step_id,
        animated: true,
    }));

    // Return a default viewport if none is stored/provided
    const viewport: Viewport = { x: 0, y: 0, zoom: 1 };

    return { nodes, edges, viewport };
};


const PipelineEditorPage: React.FC = () => {
  const { pipelineId } = useParams<{ pipelineId: string }>();
  const navigate = useNavigate();
  const queryClient = useQueryClient();
  // Re-introduce state for the local storage entry for rename/version list
  const [pipelineEntry, setPipelineEntry] = useState<PipelineStorageEntry | null>(null);
  const [currentVersionId, setCurrentVersionId] = useState<string | undefined>(undefined);
  const [displayNodes, setDisplayNodes] = useState<Node[] | undefined>(undefined);
  const [displayEdges, setDisplayEdges] = useState<Edge[] | undefined>(undefined);
  const [displayViewport, setDisplayViewport] = useState<Viewport | undefined>(undefined);
  const reactFlowInstanceRef = useRef<ReactFlowInstance | null>(null);

  // Fetch pipeline details using React Query
  const { data: pipelineData, isLoading: isLoadingPipeline, error: pipelineError, isError } = useQuery<PipelineDefinition | null, Error>({
    queryKey: ['pipelineDetail', pipelineId],
    queryFn: () => pipelineId ? fetchPipelineDetail(pipelineId) : Promise.resolve(null),
    enabled: !!pipelineId,
    retry: 1,
  });

  // TODO: Fetch step definitions separately using useQuery
  const stepDefinitions = hardcodedStepDefinitions; // Use hardcoded as fallback

  // Effect to update display state based on fetched data or local storage
  useEffect(() => {
    const entry = pipelineId ? getPipelineEntry(pipelineId) : null; // Get local entry for name/versions
    setPipelineEntry(entry || null);

    if (pipelineData) {
        const flowData = getFlowDataFromPipeline(pipelineData, stepDefinitions);
        setDisplayNodes(flowData?.nodes || []);
        setDisplayEdges(flowData?.edges || []);
        setDisplayViewport(flowData?.viewport || { x: 0, y: 0, zoom: 1 }); // Ensure viewport is set
        setCurrentVersionId(pipelineData.version_id); // Set current version from fetched data
    } else if (!isLoadingPipeline && pipelineId) {
        // Fallback to local storage if API fails or pipeline is new
        if (entry && entry.versions.length > 0) {
            const latestVersion = entry.versions[0];
            const flowData = latestVersion.flowData;
            setDisplayNodes(flowData.nodes);
            setDisplayEdges(flowData.edges);
            setDisplayViewport(flowData.viewport); // Viewport should exist here
            setCurrentVersionId(latestVersion.versionId);
        } else if (entry) { // New pipeline (exists locally, no versions)
             setDisplayNodes(createDefaultNodes(stepDefinitions)); // Pass stepDefs
             setDisplayEdges([]);
             setDisplayViewport({ x: 0, y: 0, zoom: 1 }); // Default viewport
             setCurrentVersionId(undefined);
        } else if (isError) {
            toast.error(`Failed to load pipeline: ${pipelineError?.message || 'Unknown error'}`);
            navigate('/pipelines');
        } else { // Not loading, no error, no data, no entry -> invalid ID
             toast.error("Pipeline not found.");
             navigate('/pipelines');
        }
    }
  }, [pipelineData, isLoadingPipeline, pipelineId, navigate, isError, pipelineError, stepDefinitions]); // Add stepDefinitions


  const handleSave = useCallback(() => {
    if (!pipelineId || !reactFlowInstanceRef.current || !pipelineEntry) return;
    const currentFlowData: FlowData = {
      nodes: reactFlowInstanceRef.current.getNodes(),
      edges: reactFlowInstanceRef.current.getEdges(),
      viewport: reactFlowInstanceRef.current.getViewport(),
    };
    const newVersion: PipelineVersion = {
      versionId: Date.now().toString(),
      timestamp: new Date().toISOString(),
      flowData: currentFlowData,
    };
    // TODO: Replace with useMutation calling savePipelineVersion API
    addPipelineVersion(pipelineId, newVersion);
    const updatedEntry = getPipelineEntry(pipelineId);
    setPipelineEntry(updatedEntry || null); // Update local entry state
    // Update display state immediately
    setDisplayNodes(newVersion.flowData.nodes);
    setDisplayEdges(newVersion.flowData.edges);
    setDisplayViewport(newVersion.flowData.viewport);
    setCurrentVersionId(newVersion.versionId);
    toast.success(`Pipeline '${pipelineEntry.name}' saved successfully!`);
  }, [pipelineId, pipelineEntry, reactFlowInstanceRef]);

  const handleRename = useCallback((id: string, newName: string): boolean => {
    // TODO: Replace with useMutation calling renamePipelineApi
    const success = renamePipeline(id, newName);
    if (success) {
        setPipelineEntry(prev => prev ? { ...prev, name: newName } : null); // Update local entry state
        queryClient.invalidateQueries({ queryKey: ['pipelineDetail', pipelineId] });
        queryClient.invalidateQueries({ queryKey: ['pipelines'] });
    }
    return success;
  }, [pipelineId, queryClient]);

  const handleSelectVersion = useCallback((versionId: string) => {
    if (!pipelineId) return;
    // TODO: Refetch specific version via API if needed
    const entry = getPipelineEntry(pipelineId); // Still using local for mock version switching
    const selectedVersion = entry?.versions.find(v => v.versionId === versionId);
    if (selectedVersion?.flowData) {
      setCurrentVersionId(selectedVersion.versionId);
      setDisplayNodes(selectedVersion.flowData.nodes);
      setDisplayEdges(selectedVersion.flowData.edges);
      setDisplayViewport(selectedVersion.flowData.viewport); // Load viewport too
      toast.info(`Loaded version from ${new Date(selectedVersion.timestamp).toLocaleString()}`);
    } else {
      toast.error("Selected version data not found.");
    }
  }, [pipelineId]); // Removed pipelineEntry dependency as we refetch it

  const handleFlowInit = useCallback((instance: ReactFlowInstance) => {
    reactFlowInstanceRef.current = instance;
  }, []);

  const handleRun = useCallback(() => {
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


  if (isLoadingPipeline || displayNodes === undefined) {
    return (
        <div className="flex h-full items-center justify-center p-6">
            <Loader2Icon className="mr-2 h-6 w-6 animate-spin" /> Loading pipeline...
        </div>
    );
  }

  return (
    <ReactFlowProvider>
      <div className="flex flex-col h-full">
        <PipelineEditorTopBar
          pipelineId={pipelineId}
          pipelineName={pipelineData?.name || pipelineEntry?.name || 'Loading...'} // Use fetched or local name
          onSave={handleSave}
          onRename={handleRename}
          versions={pipelineEntry?.versions || []} // Use local entry for version list
          selectedVersionId={currentVersionId}
          onSelectVersion={handleSelectVersion}
        />
         <div className="p-2 border-b border-border bg-card flex justify-end">
             <Button onClick={handleRun} size="sm" className={cn(buttonFocusStyle)}>
                 <PlayIcon className="mr-2 h-4 w-4" /> Run Pipeline (Simulated)
             </Button>
         </div>
        <div className="flex flex-grow overflow-hidden">
          <PipelineEditorFlow
            key={currentVersionId || 'initial'}
            initialNodes={displayNodes}
            initialEdges={displayEdges}
            initialViewport={displayViewport}
            onFlowInit={handleFlowInit}
          />
          <PipelineSidebar />
        </div>
      </div>
    </ReactFlowProvider>
  );
};

export default PipelineEditorPage;