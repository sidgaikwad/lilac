import { useParams } from 'react-router-dom';
import {
  getProjectQuery,
  useGetDataset,
  useListDatasetS3Prefixes,
} from '@/services';
import {
  Container,
  ContainerAction,
  ContainerContent,
  ContainerDescription,
  ContainerHeader,
  ContainerTitle,
} from '@/components/ui/container';
import Breadcrumbs from '@/components/common/breadcrumbs';
import { useSuspenseQuery } from '@tanstack/react-query';
import { toast } from 'sonner';
import { FileBrowser } from '../components/file-browser';

function DataSetDetailPage() {
  const { projectId, datasetId } = useParams<{
    projectId: string;
    datasetId: string;
  }>();

  const { data: project } = useSuspenseQuery(getProjectQuery(projectId));
  const { data: dataset } = useGetDataset({
    datasetId,
    onError: (error) =>
      toast.error('Failed to load dataset', {
        description: error.error,
      }),
  });
  const { data: _objects } = useListDatasetS3Prefixes({
    datasetId,
    params: {
      prefix: '',
    },
    onSuccess: (objects) => console.log(objects),
  });

  return (
    <Container>
      <ContainerHeader>
        <div className='flex-1 shrink-0 grow-0 basis-full pb-4'>
          <Breadcrumbs
            breadcrumbs={[
              {
                content: 'Projects',
                link: `/`,
              },
              {
                content: project?.name ?? projectId,
                link: `/projects/${projectId}`,
              },
              {
                content: 'Datasets',
                link: `/projects/${projectId}/datasets`,
              },
              {
                content: dataset?.name,
                link: `/projects/${projectId}/datasets/${datasetId}`,
              },
            ]}
          />
        </div>
        <ContainerTitle>
          {dataset?.name}
          <ContainerDescription>Browse your S3 bucket</ContainerDescription>
        </ContainerTitle>
        <ContainerAction></ContainerAction>
      </ContainerHeader>

      <ContainerContent>
        {dataset && (
          <FileBrowser
            bucketName={dataset?.datasetSource.bucketName}
            datasetId={dataset.id}
            rootPrefix=''
            name='/'
          />
        )}
      </ContainerContent>
    </Container>
  );
}

export default DataSetDetailPage;
