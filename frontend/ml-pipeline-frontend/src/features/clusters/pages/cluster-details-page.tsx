import { getClusterQuery } from '@/services';
import {
  Container,
  ContainerAction,
  ContainerContent,
  ContainerDescription,
  ContainerHeader,
  ContainerTitle,
} from '@/components/ui/container';
import Breadcrumbs from '@/components/common/breadcrumbs';
import { useParams } from 'react-router-dom';
import { useSuspenseQuery } from '@tanstack/react-query';
import { Tabs, TabsList, TabsTrigger, TabsContent } from '@/components/ui/tabs';

function ClusterDetailsPage() {
  const { clusterId } = useParams<{
    clusterId: string;
  }>();

  const { data: cluster } = useSuspenseQuery(getClusterQuery(clusterId));

  return (
    <Container>
      <ContainerHeader>
        <div className='flex-1 shrink-0 grow-0 basis-full pb-4'>
          <Breadcrumbs
            breadcrumbs={[
              {
                content: 'Clusters',
                link: '/clusters',
              },
              {
                content: cluster.clusterName,
                link: `/clusters/${clusterId}`,
              },
            ]}
          />
        </div>
        <ContainerTitle>
          {cluster.clusterName}
          <ContainerDescription></ContainerDescription>
        </ContainerTitle>
        <ContainerAction></ContainerAction>
      </ContainerHeader>

      <ContainerContent>
        <Tabs defaultValue='overview' className='w-full'>
          <TabsList className='max-w-[400px]'>
            <TabsTrigger value='overview'>Overview</TabsTrigger>
            <TabsTrigger value='jobs'>Jobs</TabsTrigger>
          </TabsList>
          <TabsContent value='overview'>
            TODO
          </TabsContent>
          <TabsContent value='jobs'>
            TODO
          </TabsContent>
        </Tabs>
      </ContainerContent>
    </Container>
  );
}

export default ClusterDetailsPage;
