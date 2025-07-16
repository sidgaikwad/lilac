import { useState } from 'react';
import { useParams } from 'react-router-dom';
import { Skeleton } from '@/components/ui/skeleton';
import { Card, CardContent, CardHeader } from '@/components/ui/card';
import CreateDatasetModal from '../components/connect-dataset-modal';
import { getProjectQuery } from '@/services';
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
import { toast } from '@/components/toast';
import EmptyCardSection from '@/components/common/empty-card-section';
import { DatasetCard } from '../components/dataset-card';

function DataSetsPage() {
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
        <div className='flex-1 shrink-0 grow-0 basis-full pb-4'>
          <Breadcrumbs
            breadcrumbs={[
              {
                content: 'Projects',
                link: `/`,
              },
              {
                content: project?.projectName ?? projectId,
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
          <div className='space-y-4'>
            {[...Array(3)].map((_, i) => (
              <Card key={i}>
                <CardHeader>
                  <Skeleton className='h-6 w-1/2' />
                </CardHeader>
                <CardContent>
                  <div className='grid grid-cols-2 gap-4 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5'>
                    {[...Array(5)].map((_, j) => (
                      <Skeleton
                        key={j}
                        className='bg-muted aspect-square h-auto w-full'
                      />
                    ))}
                  </div>
                </CardContent>
              </Card>
            ))}
          </div>
        )}
        {datasets !== undefined && datasets.length > 0 ? (
          datasets.map((dataset) => (
            <DatasetCard
              key={dataset.datasetId}
              projectId={projectId!}
              dataset={dataset}
            />
          ))
        ) : (
          <EmptyCardSection
            title={'No datasets'}
            buttonText={'Create Dataset'}
            onClick={() => setOpen(true)}
          />
        )}
      </ContainerContent>
    </Container>
  );
}

export default DataSetsPage;
