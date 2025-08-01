import {
  Container,
  ContainerAction,
  ContainerContent,
  ContainerDescription,
  ContainerHeader,
  ContainerTitle,
} from '@/components/ui/container';
import Breadcrumbs from '@/components/common/breadcrumbs';
import { useLocation, useNavigate, useParams } from 'react-router-dom';
import { useSuspenseQuery } from '@tanstack/react-query';
import { Tabs, TabsList, TabsTrigger, TabsContent } from '@/components/ui/tabs';
import { ClusterOverview } from '../components/cluster-overview';
import { getClusterInfoQuery } from '@/services/clusters/get-cluster-info.query';
import { ClusterJobs } from '../components/cluster-jobs';
import { useEffect } from 'react';
import { ClusterApiKeys } from '../components/cluster-api-keys';

function ClusterDetailsPage() {
  const location = useLocation();
  const navigate = useNavigate();
  const { clusterId } = useParams<{
    clusterId: string;
  }>();

  const { data: cluster } = useSuspenseQuery(getClusterInfoQuery(clusterId));

  useEffect(() => {
    if (!location.hash) {
      navigate('#overview');
    }
  }, [location.hash, navigate]);

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
        <Tabs
          onValueChange={(value) => navigate(`#${value}`)}
          value={location.hash.slice(1)}
          defaultValue='overview'
          className='w-full'
        >
          <TabsList className='max-w-[400px]'>
            <TabsTrigger value='overview'>Overview</TabsTrigger>
            <TabsTrigger value='jobs'>Jobs</TabsTrigger>
            <TabsTrigger value='keys'>API Keys</TabsTrigger>
          </TabsList>
          <TabsContent value='overview'>
            <ClusterOverview cluster={cluster} />
          </TabsContent>
          <TabsContent value='jobs'>
            <ClusterJobs clusterId={clusterId} />
          </TabsContent>
          <TabsContent value='keys'>
            <ClusterApiKeys clusterId={clusterId} />
          </TabsContent>
        </Tabs>
      </ContainerContent>
    </Container>
  );
}

export default ClusterDetailsPage;
