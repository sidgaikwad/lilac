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
import { ClusterOverview } from '../components/cluster-overview';
import { getClusterInfoQuery } from '@/services/clusters/get-cluster-info.query';
import { ClusterJobs } from '../components/cluster-jobs';
import { useEffect } from 'react';
import { ClusterApiKeys } from '../components/cluster-api-keys';
import { Tabs } from '@/components/common/tabs';

function ClusterDetailsPage() {
  const location = useLocation();
  const navigate = useNavigate();
  const { clusterId } = useParams<{
    clusterId: string;
  }>();

  const { data: cluster } = useSuspenseQuery(getClusterInfoQuery(clusterId));

  useEffect(() => {
    if (!location.hash) {
      navigate('#overview', {
        replace: true,
      });
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
          defaultTab='overview'
          items={[
            {
              id: 'overview',
              content: <ClusterOverview cluster={cluster} />,
            },
            {
              id: 'jobs',
              content: <ClusterJobs clusterId={clusterId} />,
            },
            {
              id: 'keys',
              content: <ClusterApiKeys clusterId={clusterId} />,
            },
          ]}
        />
      </ContainerContent>
    </Container>
  );
}

export default ClusterDetailsPage;
