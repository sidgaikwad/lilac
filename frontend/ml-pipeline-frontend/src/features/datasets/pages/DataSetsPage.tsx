import React, { useState } from 'react';
import { useParams } from 'react-router-dom';
import {
  useListJobOutputs,
  useListJobOutputImages,
} from '@/services/controlplane-api/useJobOutputs.hook';
import { Skeleton } from '@/components/ui/skeleton';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import CreateDatasetModal from '../components/CreateDatasetModal';

const JobOutputImagesGrid: React.FC<{ jobId: string }> = ({ jobId }) => {
  const {
    data: jobOutputImages,
    isLoading,
    isError,
    error,
  } = useListJobOutputImages(jobId);


  if (isLoading) {
    return (
      <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-4">
        {[...Array(5)].map((_, i) => (
          <Skeleton key={i} className="aspect-square w-full h-auto" />
        ))}
      </div>
    );
  }

  if (isError) {
    return <p className="text-red-500">Error loading images: {error?.error}</p>;
  }

  if (!jobOutputImages || jobOutputImages.images.length === 0) {
    return <p className="text-muted-foreground">No images found for this job output.</p>;
  }

  return (
    <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-4">
      {jobOutputImages.images.map((imageName) => (
        <div key={imageName} className="aspect-square border rounded-lg overflow-hidden">
          <img
            src={`http://localhost:3000/static/job_outputs/${jobId}/output/${imageName}`}
            alt={imageName}
            className="w-full h-full object-cover"
          />
        </div>
      ))}
    </div>
  );
};

const DataSetsPage: React.FC = () => {
  const { projectId } = useParams<{ projectId: string }>(); // Assuming projectId is in the URL
  const {
    data: jobOutputs,
    isLoading,
    isError,
    error,
  } = useListJobOutputs(projectId);
  const [isOpen, setOpen] = useState(false);

  return (
    <div className="container mx-auto p-4 md:p-6 lg:p-8">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-semibold">Processed Data Sets (Job Outputs)</h1>
        <CreateDatasetModal projectId={projectId ?? ''} isOpen={isOpen} setOpen={setOpen} />
      </div>

      {isLoading && (
        <div className="space-y-4">
          {[...Array(3)].map((_, i) => (
            <Card key={i}>
              <CardHeader>
                <Skeleton className="h-6 w-1/2" />
              </CardHeader>
              <CardContent>
                <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-4">
                  {[...Array(5)].map((_, j) => (
                    <Skeleton key={j} className="aspect-square w-full h-auto" />
                  ))}
                </div>
              </CardContent>
            </Card>
          ))}
        </div>
      )}

      {isError && (
        <p className="text-red-500">
          Error loading job outputs: {error?.error}
        </p>
      )}

      {!isLoading && !isError && (!jobOutputs || jobOutputs.length === 0) && (
        <p className="text-center text-muted-foreground py-8">
          No processed datasets found for this project.
        </p>
      )}

      {!isLoading && !isError && jobOutputs && jobOutputs.length > 0 && (
        <div className="space-y-6">
          {jobOutputs.map((jobOutput) => (
            <Card key={jobOutput.jobId}>
              <CardHeader>
                <CardTitle>Job ID: {jobOutput.jobId}</CardTitle>
                {jobOutput.inputDatasetName && (
                  <p className="text-sm text-muted-foreground">
                    Input: {jobOutput.inputDatasetName}
                  </p>
                )}
                {jobOutput.completedAt && (
                  <p className="text-sm text-muted-foreground">
                    Completed: {new Date(jobOutput.completedAt).toLocaleString()}
                  </p>
                )}
              </CardHeader>
              <CardContent>
                <JobOutputImagesGrid jobId={jobOutput.jobId} />
              </CardContent>
            </Card>
          ))}
        </div>
      )}
    </div>
  );
};

export default DataSetsPage;
