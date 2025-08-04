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
import { useEffect } from 'react';
import { Tabs } from '@/components/common/tabs';
import { NodeOverview } from '../components/node-overview';
import { getClusterNodeQuery } from '@/services/clusters/get-cluster-node.query';

function NodeDetailsPage() {
  const location = useLocation();
  const navigate = useNavigate();
  const { nodeId } = useParams<{
    nodeId: string;
  }>();

  const { data: node } = useSuspenseQuery(getClusterNodeQuery(nodeId));

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
                content: 'Nodes',
              },
              {
                content: 'Details',
                link: `/nodes/${nodeId}`,
              },
            ]}
          />
        </div>
        <ContainerTitle>
          {node.id}
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
              content: <NodeOverview node={node} />,
            },
          ]}
        />
      </ContainerContent>
    </Container>
  );
}

export default NodeDetailsPage;
