import { Card, DataTable, KeyValueDisplay } from '@/components/common';
import { Link } from '@/components/common/link';
import { Badge } from '@/components/ui/badge';
import { useListClusters } from '@/services';
import { Queue } from '@/types';

export interface QueueOverviewProps {
  queue: Queue;
}

export function QueueOverview(props: QueueOverviewProps) {
  const { queue } = props;
  const { data: clusters } = useListClusters();

  const queueClusters = (clusters ?? []).filter((cluster) =>
    queue.clusterTargets.includes(cluster.clusterId)
  );

  return (
    <Card
      className='w-full'
      title={queue.name}
      content={
        <div className='space-y-4'>
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
            ]}
          />
          <p className='text-gray-text-muted font-medium'>Cluster Targets:</p>
          <DataTable
            withHeader={false}
            columns={[
              {
                header: 'Cluster Name',
                accessorKey: 'clusterName',
                cell: ({ row, cell }) => (
                  <Link to={`/clusters/${row.original.clusterId}`}>
                    {cell.renderValue() as string}
                  </Link>
                ),
              },
              {
                accessorKey: 'totalNodes',
                header: 'Total Nodes',
                cell: ({ cell }) => {
                  return (
                    <Badge color='gray' variant='secondary'>
                      {cell.renderValue() as string}
                    </Badge>
                  );
                },
              },
              {
                accessorKey: 'busyNodes',
                header: 'Busy Nodes',
                cell: ({ cell }) => {
                  return (
                    <Badge color='red' variant='secondary'>
                      {cell.renderValue() as string}
                    </Badge>
                  );
                },
              },
              {
                accessorKey: 'totalRunningJobs',
                header: 'Running Jobs',
                cell: ({ cell }) => {
                  return (
                    <Badge color='blue' variant='secondary'>
                      {cell.renderValue() as string}
                    </Badge>
                  );
                },
              },
            ]}
            data={queueClusters}
          />
        </div>
      }
    />
  );
}
