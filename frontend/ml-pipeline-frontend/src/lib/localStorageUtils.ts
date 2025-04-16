import { Node, Edge, Viewport } from 'reactflow';

// --- Types for Local Storage Structure ---

export interface FlowData {
  nodes: Node[];
  edges: Edge[];
  viewport: Viewport;
}

export interface PipelineVersion {
  versionId: string; // Timestamp-based ID (e.g., Date.now().toString())
  timestamp: string; // ISO string timestamp
  flowData: FlowData;
}

export interface PipelineStorageEntry {
  id: string; // UUID
  name: string;
  // Store versions chronologically, newest first might be convenient
  versions: PipelineVersion[];
}

// Root structure in localStorage
interface LocalStorageStructure {
  pipelines: Record<string, PipelineStorageEntry>; // Keyed by pipelineId
}

const LOCAL_STORAGE_KEY = 'pipelineData';

// --- Utility Functions ---

/**
 * Retrieves the entire pipeline data structure from local storage.
 * Handles parsing and returns a default structure if not found or invalid.
 */
export function getStoredPipelines(): LocalStorageStructure {
  try {
    const storedData = localStorage.getItem(LOCAL_STORAGE_KEY);
    if (storedData) {
      const parsed = JSON.parse(storedData);
      // Basic validation: ensure 'pipelines' property exists and is an object
      if (parsed && typeof parsed === 'object' && typeof parsed.pipelines === 'object') {
        return parsed as LocalStorageStructure;
      }
    }
  } catch (error) {
    console.error("Error reading pipeline data from local storage:", error);
  }
  // Return default structure if nothing found or error occurred
  return { pipelines: {} };
}

/**
 * Saves the entire pipeline data structure to local storage.
 */
export function saveStoredPipelines(data: LocalStorageStructure): void {
  try {
    localStorage.setItem(LOCAL_STORAGE_KEY, JSON.stringify(data));
  } catch (error) {
    console.error("Error saving pipeline data to local storage:", error);
    // Optionally notify the user
  }
}

/**
 * Retrieves a specific pipeline entry by its ID.
 */
export function getPipelineEntry(pipelineId: string): PipelineStorageEntry | undefined {
  const data = getStoredPipelines();
  return data.pipelines[pipelineId];
}

/**
 * Saves or updates a specific pipeline entry.
 */
export function savePipelineEntry(pipelineEntry: PipelineStorageEntry): void {
  const data = getStoredPipelines();
  data.pipelines[pipelineEntry.id] = pipelineEntry;
  saveStoredPipelines(data);
}

/**
 * Adds a new version to a specific pipeline entry.
 */
export function addPipelineVersion(pipelineId: string, newVersion: PipelineVersion): void {
  const data = getStoredPipelines();
  const pipelineEntry = data.pipelines[pipelineId];
  if (pipelineEntry) {
    // Add to the beginning of the array (newest first)
    pipelineEntry.versions.unshift(newVersion);
    saveStoredPipelines(data);
  } else {
    console.error(`Pipeline with ID ${pipelineId} not found for adding version.`);
  }
}

/**
 * Renames a specific pipeline.
 */
export function renamePipeline(pipelineId: string, newName: string): boolean {
    const data = getStoredPipelines();
    const pipelineEntry = data.pipelines[pipelineId];
    if (pipelineEntry) {
        pipelineEntry.name = newName;
        saveStoredPipelines(data);
        return true;
    } else {
        console.error(`Pipeline with ID ${pipelineId} not found for renaming.`);
        return false;
    }
}

/**
 * Deletes a specific pipeline entry.
 */
export function deletePipelineEntry(pipelineId: string): boolean {
    const data = getStoredPipelines();
    if (data.pipelines[pipelineId]) {
        delete data.pipelines[pipelineId];
        saveStoredPipelines(data);
        return true;
    } else {
        console.warn(`Pipeline with ID ${pipelineId} not found for deletion.`);
        return false;
    }
}


/**
 * Lists basic info for all stored pipelines (for dashboard).
 */
export function listStoredPipelines(): { id: string; name: string; lastModified: string }[] {
    const data = getStoredPipelines();
    return Object.values(data.pipelines).map(entry => ({
        id: entry.id,
        name: entry.name,
        // Get timestamp of the latest version (assuming newest is first)
        lastModified: entry.versions[0]?.timestamp || new Date(0).toISOString(),
    })).sort((a, b) => new Date(b.lastModified).getTime() - new Date(a.lastModified).getTime()); // Sort by date descending
}