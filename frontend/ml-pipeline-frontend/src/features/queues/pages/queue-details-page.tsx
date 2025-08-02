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
import { getQueueQuery } from '@/services/queues/get-queue.query';
import { QueueOverview } from '../components/queue-overview';
import { Tabs } from '@/components/common/tabs';
import { QueueJobs } from '../components/queue-jobs';

function ClusterDetailsPage() {
  const { queueId } = useParams<{
    queueId: string;
  }>();

  const { data: queue } = useSuspenseQuery(getQueueQuery(queueId));

  return (
    <Container>
      <ContainerHeader>
        <div className='flex-1 shrink-0 grow-0 basis-full pb-4'>
          <Breadcrumbs
            breadcrumbs={[
              {
                content: 'Queues',
                link: '/queues',
              },
              {
                content: queue.name,
                link: `/queues/${queueId}`,
              },
            ]}
          />
        </div>
        <ContainerTitle>
          {queue.name}
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
              content: <QueueOverview queue={queue} />,
            },
            {
              id: 'jobs',
              content: <QueueJobs queueId={queueId} />,
            },
          ]}
        />
      </ContainerContent>
    </Container>
  );
}

export default ClusterDetailsPage;
