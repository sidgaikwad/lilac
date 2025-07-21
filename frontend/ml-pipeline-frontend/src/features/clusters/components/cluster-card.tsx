import { ClusterSummary } from '@/types';
import { useNavigate } from 'react-router-dom';
import { Card } from '@/components/common/card';
import { EksLogo } from '@/icons/eks';
import { GkeLogo } from '@/icons/gke';

export interface ClusterCardProps {
  cluster: ClusterSummary;
}

function getClusterIcon(clusterType: string) {
  switch (clusterType) {
    case 'aws_eks':
      return <EksLogo className='size-12 rounded-sm' />;
    case 'gcp_gke':
      return <GkeLogo className='size-12 rounded-sm' />;
    default:
      return undefined;
  }
}

export function ClusterCard(props: ClusterCardProps) {
  const navigate = useNavigate();
  return (
    <Card
      className='hover:bg-accent-secondary h-fit w-fit min-w-xs cursor-pointer'
      onClick={() => navigate(`/clusters/${props.cluster.clusterId}`)}
      title={props.cluster.clusterName}
      description={props.cluster.clusterDescription}
      icon={getClusterIcon(props.cluster.clusterType)}
    />
  );
}
