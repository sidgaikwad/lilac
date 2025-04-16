import React, { useState, useEffect, useCallback, useRef } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { ReactFlowProvider, ReactFlowInstance, Node, Edge, Viewport } from 'reactflow';
import { hardcodedStepDefinitions } from '@/config/stepDefinitions';
import PipelineSidebar from '../components/PipelineSidebar';
import PipelineEditorFlow from '../components/PipelineEditorFlow';
import PipelineEditorTopBar from '../components/PipelineEditorTopBar';
import { getPipelineEntry, addPipelineVersion, renamePipeline, PipelineVersion, FlowData, PipelineStorageEntry } from '@/lib/localStorageUtils';
import { toast } from 'sonner';
import { Button } from '@/components/ui/button';
import { PlayIcon } from 'lucide-react';
import { cn } from '@/lib/utils';

const buttonFocusStyle = "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 dark:focus-visible:ring-offset-gray-950";

const createDefaultNodes = (): Node[] => {
  const inputDef = hardcodedStepDefinitions.find(def => def.category === 'Input');
  const outputDef = hardcodedStepDefinitions.find(def => def.category === 'Output');
  const nodes: Node[] = [];
  if (inputDef) {
    const defaultParams = inputDef.parameters.reduce((acc, param) => { if (param.defaultValue !== undefined) acc[param.name] = param.defaultValue; return acc; }, {} as Record<string, any>);
    nodes.push({ id: 'input_node', type: 'pipelineNode', position: { x: 50, y: 150 }, data: { label: inputDef.label, stepType: inputDef.type, parameters: defaultParams, stepDefinition: inputDef } });
  } else { console.warn("Default 'Input' step definition not found."); }
  if (outputDef) {
     const defaultParams = outputDef.parameters.reduce((acc, param) => { if (param.defaultValue !== undefined) acc[param.name] = param.defaultValue; return acc; }, {} as Record<string, any>);
    nodes.push({ id: 'output_node', type: 'pipelineNode', position: { x: 650, y: 150 }, data: { label: outputDef.label, stepType: outputDef.type, parameters: defaultParams, stepDefinition: outputDef } });
  } else { console.warn("Default 'Output' step definition not found."); }
  return nodes;
};

const PipelineEditorPage: React.FC = () => {
  const { pipelineId } = useParams<{ pipelineId: string }>();
  const navigate = useNavigate();
  const [pipelineEntry, setPipelineEntry] = useState<PipelineStorageEntry | null>(null);
  const [currentVersionId, setCurrentVersionId] = useState<string | undefined>(undefined);
  const [displayNodes, setDisplayNodes] = useState<Node[] | undefined>(undefined);
  const [displayEdges, setDisplayEdges] = useState<Edge[] | undefined>(undefined);
  const [displayViewport, setDisplayViewport] = useState<Viewport | undefined>(undefined);
  const [isLoading, setIsLoading] = useState(true);
  const reactFlowInstanceRef = useRef<ReactFlowInstance | null>(null);

  useEffect(() => {
    if (!pipelineId) { navigate('/pipelines'); return; }
    // TODO: API Call - GET /pipeline/{pipelineId} (replace getPipelineEntry)
    // This API should return the pipeline details including all versions or just the latest?
    // Assuming for now it returns the entry structure similar to localStorageUtils
    const entry = getPipelineEntry(pipelineId);
    if (!entry) {
        toast.error("Pipeline not found.");
        navigate('/pipelines');
        return;
    }

    setPipelineEntry(entry);
    const latestVersion = entry.versions[0];
    setCurrentVersionId(latestVersion?.versionId);

    if (latestVersion?.flowData) {
      setDisplayNodes(latestVersion.flowData.nodes);
      setDisplayEdges(latestVersion.flowData.edges);
      setDisplayViewport(latestVersion.flowData.viewport);
    } else {
      setDisplayNodes(createDefaultNodes());
      setDisplayEdges([]);
      setDisplayViewport(undefined);
    }
    setIsLoading(false);

  }, [pipelineId, navigate]);

  const handleSave = useCallback(() => {
    if (!pipelineId || !reactFlowInstanceRef.current || !pipelineEntry) return;
    const currentFlowData: FlowData = {
      nodes: reactFlowInstanceRef.current.getNodes(),
      edges: reactFlowInstanceRef.current.getEdges(),
      viewport: reactFlowInstanceRef.current.getViewport(),
    };
    const newVersion: PipelineVersion = {
      versionId: Date.now().toString(), // API should generate version ID
      timestamp: new Date().toISOString(),
      flowData: currentFlowData,
    };
    // TODO: API Call - POST /pipeline/{pipelineId}/version (or similar endpoint) with currentFlowData
    addPipelineVersion(pipelineId, newVersion); // Save locally for now
    const updatedEntry = getPipelineEntry(pipelineId);
    setPipelineEntry(updatedEntry || null);
    setDisplayNodes(newVersion.flowData.nodes);
    setDisplayEdges(newVersion.flowData.edges);
    setDisplayViewport(newVersion.flowData.viewport);
    setCurrentVersionId(newVersion.versionId);
    toast.success(`Pipeline '${pipelineEntry.name}' saved successfully!`);
  }, [pipelineId, pipelineEntry, reactFlowInstanceRef]);

  const handleRename = useCallback((id: string, newName: string): boolean => {
    // TODO: API Call - PUT/PATCH /pipeline/{id} with { name: newName }
    const success = renamePipeline(id, newName); // Rename locally for now
    if (success) setPipelineEntry(prev => prev ? { ...prev, name: newName } : null);
    return success;
  }, []);

  const handleSelectVersion = useCallback((versionId: string) => {
    if (!pipelineEntry) return;
    // TODO: API Call - GET /pipeline/{pipelineId}/version/{versionId} (if versions aren't loaded initially)
    const selectedVersion = pipelineEntry.versions.find(v => v.versionId === versionId);
    if (selectedVersion?.flowData) {
      setIsLoading(true);
      setCurrentVersionId(selectedVersion.versionId);
      setDisplayNodes(selectedVersion.flowData.nodes);
      setDisplayEdges(selectedVersion.flowData.edges);
      setDisplayViewport(selectedVersion.flowData.viewport);
      setIsLoading(false);
      toast.info(`Loaded version from ${new Date(selectedVersion.timestamp).toLocaleString()}`);
    } else {
      toast.error("Selected version data not found.");
    }
  }, [pipelineEntry]);

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
    // TODO: API Call - POST /pipeline/{pipelineId}/run (or similar) with nodes/edges definition
  }, [pipelineId]);


  if (isLoading || !pipelineEntry) {
    return <div className="p-6">Loading pipeline data...</div>;
  }

  return (
    <ReactFlowProvider>
      <div className="flex flex-col h-full">
        <PipelineEditorTopBar
          pipelineId={pipelineId}
          pipelineName={pipelineEntry.name}
          onSave={handleSave}
          onRename={handleRename}
          versions={pipelineEntry.versions}
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
          <PipelineSidebar availableSteps={hardcodedStepDefinitions} />
        </div>
      </div>
    </ReactFlowProvider>
  );
};

export default PipelineEditorPage;