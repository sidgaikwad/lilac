import React, { useState } from 'react';
import { useParams } from 'react-router-dom';
import { Skeleton } from '@/components/ui/skeleton';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import CreateDatasetModal from '../components/CreateDatasetModal';
import { useListDatasets } from '@/services/controlplane-api/useListDatasets.hook';
import { useGetDataset } from '@/services/controlplane-api/useGetDataset.hook';

const DatasetImagesGrid: React.FC<{ projectId: string, datasetId: string }> = ({ projectId, datasetId }) => {

  const { data: files, isLoading, isError, error } = useGetDataset({ projectId, datasetId });

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
  return (
    <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-4">
      {files?.map((file) => (
        <div key={file.fileName} className="aspect-square border rounded-lg overflow-hidden">
          <img
            src={file.url}
            alt={file.fileName}
            className="w-full h-full object-cover"
            crossOrigin='anonymous'
          />
        </div>
      ))}
    </div>
  );
};

const DataSetsPage: React.FC = () => {
  const { projectId } = useParams<{ projectId: string }>();
  const {
    data: datasets,
    isLoading,
    isError,
    error,
  } = useListDatasets({ projectId });
  const [isOpen, setOpen] = useState(false);

  return (
    <div className="container mx-auto p-4 md:p-6 lg:p-8">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-semibold">Datasets</h1>
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
                    <Skeleton key={j} className="aspect-square w-full h-auto bg-muted" />
                  ))}
                </div>
              </CardContent>
            </Card>
          ))}
        </div>
      )}

      {isError && (
        <p className="text-red-500">
          Error loading datasets: {error?.error}
        </p>
      )}

      {!isLoading && !isError && (!datasets || datasets.length === 0) && (
        <p className="text-center text-muted-foreground py-8">
          No datasets found for this project.
        </p>
      )}

      {!isLoading && !isError && datasets && datasets.length > 0 && (
        <div className="space-y-6">
          {datasets.map((dataset) => (
            <Card key={dataset.datasetId}>
              <CardHeader>
                <CardTitle>{dataset.datasetName}</CardTitle>
                <p className="text-sm text-muted-foreground">
                  {dataset.description}
                </p>
              </CardHeader>
              <CardContent>
                {projectId && <DatasetImagesGrid projectId={projectId} datasetId={dataset.datasetId} />}
              </CardContent>
            </Card>
          ))}
        </div>
      )}
    </div>
  );
};

export default DataSetsPage;
