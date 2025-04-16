import React from 'react';
import { useNavigate } from 'react-router-dom';
import { Button } from '@/components/ui/button';

const DashboardPage: React.FC = () => {
  const navigate = useNavigate();

  const handleCreatePipeline = () => {
    // TODO: Call POST /pipeline API to create a new pipeline definition
    // TODO: Get the new pipeline ID from the API response
    const newPipelineId = 'new'; // Placeholder ID
    navigate(`/pipelines/${newPipelineId}`);
  };

  // TODO: Fetch pipeline list from API (e.g., GET /pipeline?org_id=...)
  const pipelines = [
    // Dummy data for now
    { id: '123', name: 'My First Image Pipeline', last_modified: '2025-04-15' },
    { id: '456', name: 'Preprocessing Experiment', last_modified: '2025-04-14' },
  ];

  return (
    <div>
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold">Pipelines Dashboard</h1>
        <Button onClick={handleCreatePipeline}>Create New Pipeline</Button>
      </div>

      {/* TODO: Replace with actual pipeline list rendering (e.g., Table or Cards) */}
      <div className="p-4 border rounded bg-gray-50 dark:bg-gray-800">
        <h2 className="font-semibold mb-2">Existing Pipelines:</h2>
        <ul>
          {pipelines.map(p => (
            <li key={p.id} className="py-1">
              {/* TODO: Make these links: navigate(`/pipelines/${p.id}`) */}
              {p.name} - (Last Modified: {p.last_modified})
            </li>
          ))}
        </ul>
        {pipelines.length === 0 && <p className="text-muted-foreground">No pipelines found.</p>}
      </div>
    </div>
  );
};

export default DashboardPage;