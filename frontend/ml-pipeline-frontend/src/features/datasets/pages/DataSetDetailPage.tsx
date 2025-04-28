import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom'; // Assuming react-router-dom is used
import { Button } from '@/components/ui/button';
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  CardDescription,
} from '@/components/ui/card';
import { ChevronDown, ChevronRight, FileText, Folder } from 'lucide-react'; // Icons

// Mock data - replace with actual data fetching based on params.id
const mockDataSetDetail = {
  id: 'ds-1',
  name: 'Customer Churn Data',
  source: 'Local Upload',
  createdAt: '2024-04-23',
  originalDataPreview: {
    type: 'csv',
    content:
      'CustomerID,Gender,SeniorCitizen,Partner,Dependents,tenure...\n1234,Male,0,Yes,No,1...',
  },
  pipelineRuns: [
    {
      runId: 'run-abc',
      pipelineName: 'Data Cleaning Pipeline v1',
      timestamp: '2024-04-24 10:00:00',
      derivedDataSet: {
        id: 'ds-1-run-abc',
        name: 'Cleaned Customer Data',
        preview: {
          type: 'csv',
          content:
            'CustomerID,Gender,SeniorCitizen,Partner,Dependents,TenureMonths...\n1234,Male,0,Yes,No,1...',
        },
        subRuns: [
          {
            runId: 'run-def',
            pipelineName: 'Feature Engineering v2',
            timestamp: '2024-04-25 11:30:00',
            derivedDataSet: {
              id: 'ds-1-run-def',
              name: 'Engineered Features',
              preview: {
                type: 'csv',
                content:
                  'CustomerID,IsMale,IsSenior,HasPartner,HasDependents,TenureCategory...\n1234,1,0,1,0,New...',
              },
              subRuns: [], // Can be nested further
            },
          },
        ],
      },
    },
    {
      runId: 'run-xyz',
      pipelineName: 'Image Resizing Pipeline',
      timestamp: '2024-04-24 11:00:00',
      derivedDataSet: {
        id: 'ds-1-run-xyz',
        name: 'Resized Images (512x512)',
        preview: {
          type: 'image_folder',
          content: 'Preview of image thumbnails...',
        },
        subRuns: [],
      },
    },
  ],
};

interface DerivedDataSetNodeProps {
  runId: string;
  pipelineName: string;
  timestamp: string;
  derivedDataSet: {
    id: string;
    name: string;
    preview: { type: string; content: string };
    subRuns: DerivedDataSetNodeProps[]; // Recursive structure
  };
  level?: number;
}

const DerivedDataSetNode: React.FC<DerivedDataSetNodeProps> = ({
  pipelineName,
  timestamp,
  derivedDataSet,
  level = 0,
}) => {
  const [isOpen, setIsOpen] = useState(true); // Default to open for visibility

  const handleToggle = () => setIsOpen(!isOpen);
  const handleViewData = () => {
    alert(
      `Mock: Viewing data for ${derivedDataSet.name} (ID: ${derivedDataSet.id})`
    );
    // TODO: Implement actual data viewing logic (e.g., open a modal, navigate)
  };

  return (
    <div
      style={{ marginLeft: `${level * 20}px` }}
      className="mt-2 border-l pl-3"
    >
      <div className="flex items-center cursor-pointer" onClick={handleToggle}>
        {derivedDataSet.subRuns.length > 0 ? (
          isOpen ? (
            <ChevronDown className="h-4 w-4 mr-1" />
          ) : (
            <ChevronRight className="h-4 w-4 mr-1" />
          )
        ) : (
          <span className="w-4 mr-1"></span> // Placeholder for alignment
        )}
        <Folder className="h-4 w-4 mr-2 text-blue-500" />
        <span className="font-medium">{derivedDataSet.name}</span>
        <span className="text-xs text-muted-foreground ml-2">
          (Run: {pipelineName} @ {timestamp})
        </span>
        <Button
          variant="ghost"
          size="sm"
          className="ml-auto"
          onClick={(e) => {
            e.stopPropagation();
            handleViewData();
          }}
        >
          View Data
        </Button>
      </div>
      {isOpen &&
        derivedDataSet.subRuns.map((subRun) => (
          <DerivedDataSetNode
            key={subRun.runId}
            {...subRun}
            level={level + 1}
          />
        ))}
      {isOpen && derivedDataSet.subRuns.length === 0 && (
        <div
          style={{ marginLeft: `${(level + 1) * 20}px` }}
          className="mt-1 text-sm text-muted-foreground pl-4"
        >
          (No further runs on this dataset)
        </div>
      )}
    </div>
  );
};

const DataSetDetailPage: React.FC = () => {
  const navigate = useNavigate(); // Hook for navigation

  // TODO: Fetch actual dataset details based on 'id'
  const dataSet = mockDataSetDetail; // Using mock data for now

  if (!dataSet) {
    return <div className="container mx-auto p-6">Dataset not found.</div>;
  }

  const handleGoBack = () => {
    navigate(-1); // Go back to the previous page (DataSetsPage)
  };

  const handleViewOriginalData = () => {
    alert(`Mock: Viewing original data for ${dataSet.name}`);
    // TODO: Implement actual data viewing logic
  };

  return (
    <div className="container mx-auto p-4 md:p-6 lg:p-8">
      <Button onClick={handleGoBack} variant="outline" className="mb-4">
        &larr; Back to Data Sets
      </Button>

      <Card className="mb-6">
        <CardHeader>
          <CardTitle>{dataSet.name}</CardTitle>
          <CardDescription>
            Source: {dataSet.source} | Created: {dataSet.createdAt} | ID:{' '}
            {dataSet.id}
          </CardDescription>
        </CardHeader>
        <CardContent>
          <h3 className="text-lg font-semibold mb-2">Original Data</h3>
          <div className="flex items-center p-3 border rounded bg-gray-50 dark:bg-gray-800">
            <FileText className="h-5 w-5 mr-3 text-gray-600 dark:text-gray-400" />
            <div className="flex-grow">
              <p className="text-sm font-medium">Original Dataset</p>
              <p className="text-xs text-muted-foreground truncate">
                Preview: {dataSet.originalDataPreview.content}
              </p>
            </div>
            <Button
              variant="outline"
              size="sm"
              onClick={handleViewOriginalData}
            >
              View Full Data
            </Button>
          </div>
        </CardContent>
      </Card>

      <h2 className="text-xl font-semibold mb-4">
        Pipeline Runs & Derived Data Sets
      </h2>
      {dataSet.pipelineRuns.length > 0 ? (
        dataSet.pipelineRuns.map((run) => (
          <DerivedDataSetNode key={run.runId} {...run} />
        ))
      ) : (
        <p className="text-muted-foreground">
          No pipeline runs have been performed using this dataset yet.
        </p>
      )}
    </div>
  );
};

export default DataSetDetailPage;
