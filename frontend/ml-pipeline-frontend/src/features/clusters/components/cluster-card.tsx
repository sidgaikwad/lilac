import { ClusterSummary } from '@/types';
import { useNavigate } from 'react-router-dom';
import { Card } from '@/components/common/card';

export interface ClusterCardProps {
  cluster: ClusterSummary;
}

export function ClusterCard(props: ClusterCardProps) {
  const navigate = useNavigate();
  return (
    <Card
      className='hover:bg-accent-secondary h-fit w-fit min-w-xs cursor-pointer'
      onClick={() => navigate(`/clusters/${props.cluster.clusterId}`)}
      title={props.cluster.clusterName}
      description={props.cluster.clusterDescription}
    />
  );
}
