import { Card, KeyValueDisplay } from '@/components/common';
import { Link } from '@/components/common/link';
import { RelativeTime } from '@/components/common/relative-time';
import { Spinner } from '@/components/common/spinner/spinner';
import { Status, StatusProps } from '@/components/common/status';
import { Routes } from '@/constants';
import { route } from '@/lib';
import { useGetCluster } from '@/services';
import { ClusterNode } from '@/types';

function getStatusType(
  nodeStatus: ClusterNode['nodeStatus']
): StatusProps['status'] {
  switch (nodeStatus) {
    case 'available':
      return 'success';
    case 'busy':
      return 'error';
  }
}

export interface NodeOverviewProps {
  node: ClusterNode;
}

export function NodeOverview(props: NodeOverviewProps) {
  const { node } = props;
  const { data: cluster, isLoading: clusterLoading } = useGetCluster({
    clusterId: node.clusterId,
  });

  return (
    <Card
      className='w-full'
      title='Node Details'
      content={
        <div className='space-y-4'>
          <KeyValueDisplay
            items={[
              {
                key: 'ID',
                value: <span className='font-mono'>{node.id}</span>,
              },
              {
                key: 'Status',
                value: (
                  <Status
                    status={getStatusType(node.nodeStatus)}
                    className='capitalize'
                  >
                    {node.nodeStatus}
                  </Status>
                ),
              },
              {
                key: 'Cluster',
                value: clusterLoading ? (
                  <Spinner className='m-1' size='xsmall' />
                ) : (
                  <Link
                    to={route(Routes.CLUSTER_DETAILS, {
                      clusterId: node.clusterId,
                    })}
                  >
                    {cluster?.clusterName}
                  </Link>
                ),
              },
              {
                key: 'Last Heartbeat',
                value: <RelativeTime date={new Date(node.lastHeartbeat)} />,
              },
              {
                key: 'CPU Manufacturer',
                value: <span>{node.cpu.manufacturer}</span>,
              },
              {
                key: 'CPU Architecture',
                value: <span>{node.cpu.architecture}</span>,
              },
              {
                key: 'CPU cores',
                value: <span>{node.cpu.millicores}m</span>,
              },
              {
                key: 'GPU Manufacturer',
                value: node.gpu ? (
                  <span>{node.gpu.manufacturer}</span>
                ) : (
                  <span>&ndash;</span>
                ),
              },
              {
                key: 'GPU',
                value: node.gpu ? (
                  <span>
                    {node.gpu?.count}x{node.gpu.model} ({node.gpu.memoryMb}GB)
                  </span>
                ) : (
                  <span>&ndash;</span>
                ),
              },
            ]}
          />
        </div>
      }
    />
  );
}
