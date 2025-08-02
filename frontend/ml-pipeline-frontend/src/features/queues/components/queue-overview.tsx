import { Card, KeyValueDisplay } from '@/components/common';
import { Link } from '@/components/common/link';
import { Queue } from '@/types';

export interface QueueOverviewProps {
  queue: Queue;
}

export function QueueOverview(props: QueueOverviewProps) {
  const { queue } = props;

  return (
    <Card
      className='w-full'
      title={queue.name}
      content={
        <div>
          <KeyValueDisplay
            items={[
              {
                key: 'ID',
                value: <span className='font-mono'>{queue.id}</span>,
              },
              {
                key: 'Priority',
                value: <span>{queue.priority}</span>,
              },
              {
                key: 'Cluster Targets',
                value: (
                  <ul className='list-disc'>
                    {queue.clusterTargets.map((target) => {
                      return (
                        <li key={target}>
                          <Link to={`/clusters/${target}`}>{target}</Link>
                        </li>
                      );
                    })}
                  </ul>
                ),
              },
            ]}
          />
        </div>
      }
    />
  );
}
