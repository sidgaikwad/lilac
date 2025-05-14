import React from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import { Button } from '@/components/ui/button';
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  CardDescription,
} from '@/components/ui/card';
import { FileText, Image as ImageIcon, AlertTriangle } from 'lucide-react'; 
import { useGetDataset } from '@/services/controlplane-api/useGetDataset.hook';
import { Spinner } from '@/components/ui';
import { Skeleton } from '@/components/ui/skeleton';


const formatBytes = (bytes: number, decimals = 2) => {
  if (bytes === 0) return '0 Bytes';
  const k = 1024;
  const dm = decimals < 0 ? 0 : decimals;
  const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
};

const DataSetDetailPage: React.FC = () => {
  const navigate = useNavigate();
  const { projectId, datasetId } = useParams<{
    projectId: string;
    datasetId: string;
  }>();

  const {
    data: files,
    isLoading,
    isError,
    error,
  } = useGetDataset({ projectId, datasetId });

  const handleGoBack = () => {
    navigate(-1);
  };

  if (isLoading) {
    return (
      <div className="container mx-auto p-4 md:p-6 lg:p-8 space-y-4">
        <Skeleton className="h-8 w-32 mb-4" /> {}
        <Skeleton className="h-24 w-full mb-6" /> {}
        <Skeleton className="h-12 w-full mb-2" /> {}
        <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
          {[...Array(4)].map((_, i) => (
            <Skeleton key={i} className="h-40 w-full" /> 
          ))}
        </div>
      </div>
    );
  }

  if (isError) {
    return (
      <div className="container mx-auto p-4 md:p-6 lg:p-8 text-center">
        <AlertTriangle className="h-12 w-12 text-destructive mx-auto mb-4" />
        <h2 className="text-xl font-semibold text-destructive mb-2">
          Error Loading Dataset
        </h2>
        <p className="text-muted-foreground mb-4">
          {error?.error || 'An unexpected error occurred.'}
        </p>
        <Button onClick={handleGoBack} variant="outline">
          &larr; Go Back
        </Button>
      </div>
    );
  }

  if (!files || files.length === 0) {
    return (
      <div className="container mx-auto p-4 md:p-6 lg:p-8">
        <Button onClick={handleGoBack} variant="outline" className="mb-4">
          &larr; Back to Data Sets
        </Button>
        <Card>
          <CardHeader>
            <CardTitle>Dataset Details</CardTitle>
            <CardDescription>Dataset ID: {datasetId}</CardDescription>
          </CardHeader>
          <CardContent>
            <p className="text-muted-foreground">
              No files found in this dataset or the dataset does not exist.
            </p>
          </CardContent>
        </Card>
      </div>
    );
  }

  
  
  const datasetDisplayName = files[0]?.fileName.split('/')[0] || datasetId || 'Dataset';


  return (
    <div className="container mx-auto p-4 md:p-6 lg:p-8">
      <Button onClick={handleGoBack} variant="outline" className="mb-4">
        &larr; Back to Data Sets
      </Button>

      <Card className="mb-6">
        <CardHeader>
          <CardTitle>{datasetDisplayName}</CardTitle>
          <CardDescription>
            Displaying {files.length} file(s) from dataset ID: {datasetId}
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
            {files.map((file) => (
              <Card key={file.fileName} className="overflow-hidden">
                <CardHeader className="p-3">
                  {file.fileType.startsWith('image/') ? (
                    <ImageIcon className="h-6 w-6 text-blue-500" />
                  ) : (
                    <FileText className="h-6 w-6 text-gray-500" />
                  )}
                  <CardTitle className="text-sm font-medium truncate pt-1" title={file.fileName}>
                    {file.fileName.split('/').pop()}
                  </CardTitle>
                </CardHeader>
                <CardContent className="p-3">
                  {file.fileType.startsWith('image/') ? (
                    <a href={file.url} target="_blank" rel="noopener noreferrer">
                      <img
                        src={file.url}
                        alt={file.fileName}
                        className="w-full h-32 object-cover rounded border hover:opacity-80 transition-opacity"
                      />
                    </a>
                  ) : (
                    <div className="w-full h-32 flex items-center justify-center bg-muted rounded border">
                      <p className="text-xs text-muted-foreground p-2">
                        No preview available for {file.fileType}
                      </p>
                    </div>
                  )}
                  <p className="text-xs text-muted-foreground mt-2">
                    Size: {formatBytes(file.size)}
                  </p>
                   <p className="text-xs text-muted-foreground">
                    Type: {file.fileType}
                  </p>
                </CardContent>
              </Card>
            ))}
          </div>
        </CardContent>
      </Card>
    </div>
  );
};

export default DataSetDetailPage;
