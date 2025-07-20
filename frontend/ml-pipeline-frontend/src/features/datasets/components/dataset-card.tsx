import DeleteDatasetModal from './delete-dataset-modal';
import { DatasetSummary } from '@/types';
import { useNavigate } from 'react-router-dom';
import { S3Icon } from '@/icons/s3';
import { SnowflakeIcon } from '@/icons/snowflake';
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
        navigate(
          `/projects/${props.projectId}/datasets/${props.dataset.datasetId}`
        )
      }
      title={props.dataset.datasetName}
      content={
        <KeyValueDisplay
          items={[
            {
              key: 'Name',
              value: props.dataset.datasetName,
            },
            {
              key: 'Description',
              value: props.dataset.datasetDescription,
            },
            {
              key: 'Source',
              value: props.dataset.sourceType,
            },
          ]}
          layout='vertical'
        />
      }
      icon={getIcon(props.dataset.sourceType)}
      action={
        <DeleteDatasetModal
          projectId={props.projectId}
          dataset={props.dataset}
        />
      }
    />
  );
}
