import apiClient from '@/services/apiClient';
import { StepDefinition } from '@/types';
import { hardcodedStepDefinitions } from '@/config/stepDefinitions'; // Use hardcoded data for mock

// TODO: Replace mock with actual API call to GET /step_definitions
export const fetchStepDefinitions = async (): Promise<StepDefinition[]> => {
    console.log("Fetching step definitions via stepDefinitionService (mocked)");
    // Simulate API delay
    await new Promise(res => setTimeout(res, 250));
    // Use hardcoded data as mock source
    const mockData = hardcodedStepDefinitions;
    console.log("Mock step definition data:", mockData);
    // Simulate potential API error
    // if (Math.random() > 0.9) throw new Error("Failed to fetch step definitions (mock error)");
    return mockData;
};