import { Skeleton } from '@/components/ui/skeleton';
import { Card, CardContent, CardHeader } from '@/components/ui/card';
import CreateClusterModal from '../components/create-cluster-modal';
import { useListClusters } from '@/services';
import {
  Container,
  ContainerAction,
  ContainerContent,
  ContainerDescription,
  ContainerHeader,
  ContainerTitle,
} from '@/components/ui/container';
import Breadcrumbs from '@/components/common/breadcrumbs';
import { toast } from '@/components/toast';
import EmptyCardSection from '@/components/common/empty-card-section';
import { ClusterCard } from '../components/cluster-card';
import { useState } from 'react';

function ClustersPage() {
  const { data: clusters, isLoading } = useListClusters({
    onError: (error) =>
      toast.error('Failed to load clusters', {
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
                content: 'Clusters',
                link: `/clusters`,
              },
            ]}
          />
        </div>
        <ContainerTitle>
          Clusters
          <ContainerDescription></ContainerDescription>
        </ContainerTitle>
        <ContainerAction>
          <CreateClusterModal isOpen={isOpen} setOpen={setOpen} />
        </ContainerAction>
      </ContainerHeader>

      <ContainerContent>
        <div className='flex flex-row space-x-4'>
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
          {clusters !== undefined && clusters.length > 0 ? (
            clusters.map((cluster) => (
              <ClusterCard key={cluster.clusterId} cluster={cluster} />
            ))
          ) : (
            <EmptyCardSection
              title={'No clusters'}
              buttonText={'Create Cluster'}
              onClick={() => setOpen(true)}
            />
          )}
        </div>
      </ContainerContent>
    </Container>
  );
}

export default ClustersPage;
