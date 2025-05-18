import { useParams } from 'react-router-dom';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { FileText, Image as ImageIcon } from 'lucide-react';
import { Skeleton } from '@/components/ui/skeleton';
import { getProjectQuery, useGetDataset } from '@/services';
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

const formatBytes = (bytes: number, decimals = 2) => {
  if (bytes === 0) return '0 Bytes';
  const k = 1024;
  const dm = decimals < 0 ? 0 : decimals;
  const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
};

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

  const getLoading = () => {
    return (
      <div className="container mx-auto space-y-4 p-4 md:p-6 lg:p-8">
        <Skeleton className="mb-4 h-8 w-32" /> {}
        <Skeleton className="mb-6 h-24 w-full" /> {}
        <Skeleton className="mb-2 h-12 w-full" /> {}
        <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4">
          {[...Array(4)].map((_, i) => (
            <Skeleton key={i} className="h-40 w-full" />
          ))}
        </div>
      </div>
    );
  };

  return (
    <Container>
      <ContainerHeader>
        <div className="flex-1 shrink-0 grow-0 basis-full pb-4">
          <Breadcrumbs
            breadcrumbs={[
              {
                content: 'Projects',
                link: `/organizations/${project?.organizationId}/projects`,
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
          <ContainerDescription>
            {dataset !== undefined ? (
              <>Displaying {dataset?.files.length} file(s) </>
            ) : undefined}
          </ContainerDescription>
        </ContainerTitle>
        <ContainerAction></ContainerAction>
      </ContainerHeader>

      <ContainerContent>
        <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4">
          {dataset === undefined
            ? getLoading()
            : dataset.files.map((file) => (
                <Card key={file.fileName} className="overflow-hidden">
                  <CardHeader className="p-3">
                    <CardTitle
                      className="flex flex-row gap-2 truncate pt-1 text-sm font-medium"
                      title={file.fileName}
                    >
                      {file.fileName.endsWith('jpg') ? (
                        <ImageIcon className="h-6 w-6 text-blue-500" />
                      ) : (
                        <FileText className="h-6 w-6 text-gray-500" />
                      )}
                      {file.fileName.split('/').pop()}
                    </CardTitle>
                  </CardHeader>
                  <CardContent className="p-3">
                    {file.fileName.endsWith('jpg') ? (
                      <a
                        href={file.url}
                        target="_blank"
                        rel="noopener noreferrer"
                      >
                        <img
                          src={file.url}
                          alt={file.fileName}
                          crossOrigin="anonymous"
                          className="h-32 w-full rounded border object-cover transition-opacity hover:opacity-80"
                        />
                      </a>
                    ) : (
                      <div className="bg-muted flex h-32 w-full items-center justify-center rounded border">
                        <p className="text-muted-foreground p-2 text-xs">
                          No preview available for {file.fileType}
                        </p>
                      </div>
                    )}
                    <p className="text-muted-foreground mt-2 text-xs">
                      Size: {formatBytes(file.size)}
                    </p>
                    <p className="text-muted-foreground text-xs">
                      Type: {file.fileName.split('.')[1]}
                    </p>
                  </CardContent>
                </Card>
              ))}
        </div>
      </ContainerContent>
    </Container>
  );
}

export default DataSetDetailPage;
