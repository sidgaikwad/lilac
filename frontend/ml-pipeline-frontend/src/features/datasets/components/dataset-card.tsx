import DeleteDatasetModal from './delete-dataset-modal';
import { DatasetSummary } from '@/types';
import { useNavigate } from 'react-router-dom';
import { S3Icon } from '@/components/icons/s3';
import { SnowflakeIcon } from '@/components/icons/snowflake';
import { Card } from '@/components/common/card';
import { KeyValueDisplay } from '@/components/common/key-value';

export interface DatasetCardProps {
  projectId: string;
  dataset: DatasetSummary;
}

function getIcon(datasetSource: string) {
  switch (datasetSource) {
    case 'S3':
      return <S3Icon className='size-16' />;
    case 'Snowflake':
      return <SnowflakeIcon className='size-16' />;
    default:
      return undefined;
  }
}

export function DatasetCard(props: DatasetCardProps) {
  const navigate = useNavigate();
  return (
    <Card
      className='hover:bg-muted h-fit w-fit cursor-pointer'
      onClick={() =>
        navigate(`/projects/${props.projectId}/datasets/${props.dataset.id}`)
      }
      title={props.dataset.name}
      content={<KeyValueDisplay data={props.dataset} layout='vertical' />}
      icon={getIcon(props.dataset.datasetSource)}
      action={
        <DeleteDatasetModal
          projectId={props.projectId}
          dataset={props.dataset}
        />
      }
    />
  );
}
