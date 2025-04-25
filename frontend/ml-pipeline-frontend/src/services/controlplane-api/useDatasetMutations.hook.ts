import { useMutation, useQueryClient } from "@tanstack/react-query";
import { QueryKeys } from "./constants";
import { simulateDelay } from "./mocks";
import { toast } from "sonner";
import { DataSetStorageEntry } from "@/lib/localStorageUtils"; // Using this type for mock structure

// --- Mock Mutation Functions ---

// Simulate creating a dataset (returns the "created" dataset)
const createDatasetMock = async (payload: Omit<DataSetStorageEntry, 'id' | 'createdAt'>): Promise<DataSetStorageEntry> => {
    console.log(`Mock API: Creating dataset "${payload.name}" for project ${payload.projectId}...`);
    await simulateDelay(400);
    const newId = `dataset-${Math.random().toString(36).substring(2, 9)}`;
    const newDataset = { ...payload, id: newId, createdAt: new Date().toISOString() };
    console.log(`Mock API: Created dataset:`, newDataset);
    // In a real API, this would return the created dataset object
    return newDataset;
};

// Simulate renaming a dataset
const renameDatasetMock = async (payload: { datasetId: string; newName: string }): Promise<boolean> => {
    console.log(`Mock API: Renaming dataset ${payload.datasetId} to "${payload.newName}"...`);
    await simulateDelay(150);
    console.log(`Mock API: Renamed dataset ${payload.datasetId}.`);
    return true; // Simulate success
};

// Simulate deleting a dataset
const deleteDatasetMock = async (datasetId: string): Promise<boolean> => {
    console.log(`Mock API: Deleting dataset ${datasetId}...`);
    await simulateDelay(550);
    console.log(`Mock API: Deleted dataset ${datasetId}.`);
    return true; // Simulate success
};


// --- Mutation Hooks ---

export function useCreateDataset() {
    const queryClient = useQueryClient();

    return useMutation({
        mutationFn: createDatasetMock,
        onSuccess: (data, variables) => {
            toast.success(`Dataset "${variables.name}" created (mock).`);
            // Invalidate the list query for the specific project
            queryClient.invalidateQueries({ queryKey: [QueryKeys.LIST_DATASETS, variables.projectId] });
        },
        onError: (error, variables) => {
            console.error("Error creating dataset (mock):", error);
            toast.error(`Failed to create dataset "${variables.name}".`);
        },
    });
}

export function useRenameDataset() {
    const queryClient = useQueryClient();

    return useMutation({
        mutationFn: renameDatasetMock,
        // Invalidate the general dataset list query key on success.
        // A real API might return the updated item or projectId allowing more specific invalidation.
        onSuccess: (data, variables) => {
            toast.success(`Dataset renamed to "${variables.newName}" (mock).`);
            console.warn("Invalidating general dataset list query after rename.");
            queryClient.invalidateQueries({ queryKey: [QueryKeys.LIST_DATASETS] });
        },
        onError: (error, variables) => {
            console.error("Error renaming dataset (mock):", error);
            toast.error(`Failed to rename dataset.`);
        },
    });
}


export function useDeleteDataset() {
    const queryClient = useQueryClient();

    return useMutation({
        mutationFn: deleteDatasetMock,
         // Invalidate the general dataset list query key on success.
        onSuccess: (data, variables) => {
            toast.success(`Dataset deleted (mock).`);
            console.warn("Invalidating general dataset list query after delete.");
            queryClient.invalidateQueries({ queryKey: [QueryKeys.LIST_DATASETS] });
        },
        onError: (error, variables) => {
            console.error("Error deleting dataset (mock):", error);
            toast.error(`Failed to delete dataset.`);
        },
    });
}