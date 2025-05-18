import React, { useState } from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import { Skeleton } from '@/components/ui/skeleton';
import {
  Card,
  CardAction,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import CreateDatasetModal from '../components/create-dataset-modal';
import { getProjectQuery, useGetDataset } from '@/services';
import { useListDatasets } from '@/services';
import {
  Container,
  ContainerAction,
  ContainerContent,
  ContainerDescription,
  ContainerHeader,
  ContainerTitle,
} from '@/components/ui/container';
import Breadcrumbs from '@/components/common/breadcrumbs';
import { useSuspenseQuery } from '@tanstack/react-query';
import { toast } from 'sonner';
import DeleteDatasetModal from '../components/delete-dataset-modal';

const DatasetImagesGrid: React.FC<{ projectId: string; datasetId: string }> = ({
  datasetId,
}) => {
  const {
    data: dataset,
    isLoading,
    isError,
    error,
  } = useGetDataset({ datasetId });

  if (isLoading) {
    return (
      <div className="grid grid-cols-2 gap-4 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5">
        {[...Array(5)].map((_, i) => (
          <Skeleton key={i} className="aspect-square h-auto w-full" />
        ))}
      </div>
    );
  }

  if (isError) {
    return <p className="text-red-500">Error loading images: {error?.error}</p>;
  }
  return (
    <div className="grid grid-cols-2 gap-4 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5">
      {dataset?.files?.map((file) => (
        <div
          key={file.fileName}
          className="aspect-square overflow-hidden rounded-lg border"
        >
          <img
            src={file.url}
            alt={file.fileName}
            className="h-full w-full object-cover"
            crossOrigin="anonymous"
          />
        </div>
      ))}
    </div>
  );
};

function DataSetsPage() {
  const navigate = useNavigate();
  const { projectId } = useParams<{ projectId: string }>();
  const { data: project } = useSuspenseQuery(getProjectQuery(projectId));
  const { data: datasets, isLoading } = useListDatasets({
    projectId,
    onError: (error) =>
      toast.error('Failed to load datasets', {
        description: error.error,
      }),
  });
  const [isOpen, setOpen] = useState(false);

  return (
    <Container>
      <ContainerHeader>
        <div className="flex-1 shrink-0 grow-0 basis-full pb-4">
          <Breadcrumbs
            breadcrumbs={[
              {
                content: 'Projects',
                link: `/organizations/${project?.organizationId}/projects`,
              },
              {
                content: project?.name ?? projectId,
                link: `/projects/${projectId}`,
              },
              {
                content: 'Datasets',
                link: `/projects/${projectId}/datasets`,
              },
            ]}
          />
        </div>
        <ContainerTitle>
          Datasets
          <ContainerDescription></ContainerDescription>
        </ContainerTitle>
        <ContainerAction>
          <CreateDatasetModal
            projectId={projectId ?? ''}
            isOpen={isOpen}
            setOpen={setOpen}
          />
        </ContainerAction>
      </ContainerHeader>

      <ContainerContent>
        {isLoading && (
          <div className="space-y-4">
            {[...Array(3)].map((_, i) => (
              <Card key={i}>
                <CardHeader>
                  <Skeleton className="h-6 w-1/2" />
                </CardHeader>
                <CardContent>
                  <div className="grid grid-cols-2 gap-4 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5">
                    {[...Array(5)].map((_, j) => (
                      <Skeleton
                        key={j}
                        className="bg-muted aspect-square h-auto w-full"
                      />
                    ))}
                  </div>
                </CardContent>
              </Card>
            ))}
          </div>
        )}
        {datasets?.map((dataset) => (
          <Card
            key={dataset.id}
            className="mb-4 cursor-pointer"
            onClick={() =>
              navigate(`/projects/${projectId}/datasets/${dataset.id}`)
            }
          >
            <CardHeader>
              <CardTitle>{dataset.name}</CardTitle>
              <CardDescription>{dataset.description}</CardDescription>
              <CardAction>
                <DeleteDatasetModal projectId={projectId!} dataset={dataset} />
              </CardAction>
            </CardHeader>
            <CardContent>
              {projectId && (
                <DatasetImagesGrid
                  projectId={projectId}
                  datasetId={dataset.id}
                />
              )}
            </CardContent>
          </Card>
        ))}
      </ContainerContent>
    </Container>
  );
};

export default DataSetsPage;
