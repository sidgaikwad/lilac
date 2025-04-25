import { Organization, Project, PipelineListItem, DataSetStorageEntry } from '@/types';
import { v4 as uuidv4 } from 'uuid';

// Re-using types like DataSetStorageEntry for simplicity in mock structure
// In a real API, these would likely be distinct types.

export const MOCK_ORGS: Organization[] = [
    { id: 'org-1', name: "Luew2's Org", created_at: new Date().toISOString() },
    { id: 'org-2', name: "Team Rocket", created_at: new Date().toISOString() },
];

export const MOCK_PROJECTS: Project[] = [
    { id: 'proj-a', name: "Data Pipeline V1", organization_id: 'org-1', created_at: new Date().toISOString() },
    { id: 'proj-b', name: "Image Analysis", organization_id: 'org-1', created_at: new Date().toISOString() },
    { id: 'proj-c', name: "Mewtwo Project", organization_id: 'org-2', created_at: new Date().toISOString() },
    { id: 'proj-d', name: "Steal Pikachu", organization_id: 'org-2', created_at: new Date().toISOString() },
];

// Mock Pipelines (using PipelineListItem structure)
export const MOCK_PIPELINES: PipelineListItem[] = [
    { id: uuidv4(), name: 'Image Resizer Pipeline', projectId: 'proj-a', lastModified: new Date(Date.now() - 86400000).toISOString() },
    { id: uuidv4(), name: 'Data Validation', projectId: 'proj-a', lastModified: new Date().toISOString() },
    { id: uuidv4(), name: 'Mewtwo Capture Sequence', projectId: 'proj-c', lastModified: new Date(Date.now() - 172800000).toISOString() },
    { id: uuidv4(), name: 'Pikachu Tracker', projectId: 'proj-d', lastModified: new Date().toISOString() },
];

// Mock Datasets (using DataSetStorageEntry structure for now)
export const MOCK_DATASETS: DataSetStorageEntry[] = [
    { id: uuidv4(), name: 'Sample Images (Project A)', orgId: 'org-1', projectId: 'proj-a', source: 'Local Upload', createdAt: new Date(Date.now() - 86400000).toISOString(), originalDataPreview: 'image1.jpg, image2.png...' },
    { id: uuidv4(), name: 'Analysis Results (Project A)', orgId: 'org-1', projectId: 'proj-a', source: 'Pipeline Output', createdAt: new Date().toISOString(), originalDataPreview: 'results.csv' },
    { id: uuidv4(), name: 'Mewtwo Training Data', orgId: 'org-2', projectId: 'proj-c', source: 'S3 Import', createdAt: new Date(Date.now() - 172800000).toISOString(), originalDataPreview: 'config.yaml, data/**' },
    { id: uuidv4(), name: 'Stolen Goods Manifest', orgId: 'org-2', projectId: 'proj-d', source: 'Manual Entry', createdAt: new Date().toISOString(), originalDataPreview: 'pokeballs.txt' },
];

// Helper to simulate API delay
export const simulateDelay = (ms: number) => new Promise(res => setTimeout(res, ms));