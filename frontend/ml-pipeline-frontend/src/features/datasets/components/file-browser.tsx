import { Button } from '@/components/ui/button';
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
} from '@/components/ui/card';
import { Separator } from '@/components/ui/separator';
import { useListDatasetS3Prefixes } from '@/services';
import useFileBrowserStore from '@/store/use-file-browser-store';
import { ChevronLeft, ChevronRight, File, Folder } from 'lucide-react';
import { useEffect, useRef } from 'react';

export function FileBrowser(props: {
  bucketName: string;
  datasetId: string;
  rootPrefix: string;
  name: string;
}) {
  const store = useFileBrowserStore((state) => ({
    currentPath: state.currentPath,
    folders: state.folders,
    onFolderBack: state.onFolderBack,
    onFolderEnter: state.onFolderEnter,
    registerFolder: state.registerFolder,
    getFolder: state.getFolder,
  }));

  const { data: folder } = useListDatasetS3Prefixes({
    datasetId: props.datasetId,
    params: {
      prefix: props.rootPrefix,
    },
  });

  useEffect(() => {
    if (folder !== undefined) {
      store.registerFolder(props.rootPrefix);
      store.onFolderEnter(props.rootPrefix);
    }
  }, [folder]);

  return (
    <Card>
      <CardHeader className='text-md font-bold'>{props.bucketName}</CardHeader>
      <CardContent>
        <Button variant='ghost' onClick={() => store.onFolderBack()}>
          <ChevronLeft />
          Back
        </Button>
        <div className='flex min-h-80 snap-x flex-row overflow-x-scroll'>
          {store.currentPath.map((path) => {
            return (
              <div className='w-full flex-none shrink-0 snap-center sm:w-1/2 md:w-1/3'>
                <FolderList
                  key={path}
                  datasetId={props.datasetId}
                  parent={path}
                  currentPath={store.currentPath}
                  onClick={(prefix) => {
                    if (store.getFolder(prefix) === undefined) {
                      store.registerFolder(
                        prefix,
                        store.folders[path].level + 1
                      );
                    }
                    store.onFolderEnter(prefix);
                  }}
                />
              </div>
            );
          })}
        </div>
      </CardContent>
      <CardFooter></CardFooter>
    </Card>
  );
}

function FolderList(props: {
  datasetId: string;
  parent: string;
  currentPath: string[];
  onClick: (prefix: string) => void;
}) {
  const { data: folder } = useListDatasetS3Prefixes({
    datasetId: props.datasetId,
    params: {
      prefix: props.parent,
    },
  });
  const ref = useRef<HTMLDivElement>(null);

  useEffect(() => {
    // scroll into view on load
    ref.current?.scrollIntoView({
      behavior: 'smooth',
    });
  }, []);

  return (
    <div ref={ref} className='flex h-full w-full flex-row'>
      <div className='flex h-full w-full flex-col'>
        {folder?.prefixes.map((prefix) => (
          <div
            key={prefix}
            data-selected={props.currentPath.includes(prefix)}
            className='group hover:bg-muted data-[selected=true]:bg-muted mx-1 flex flex-row items-center space-x-2 rounded-sm p-1'
            onClick={() => {
              props.onClick(prefix);
            }}
          >
            <Folder className='h-6 w-6 shrink-0' color='#2f81f5' />
            <span className='shrink truncate'>{prefix.split('/').at(-2)}</span>
            <ChevronRight className='invisible ml-auto min-w-2 shrink-0 group-data-[selected=true]:visible' />
          </div>
        ))}
        {folder?.objects.map((object) => (
          <div
            key={object}
            className='hover:bg-muted mx-1 flex flex-row items-center space-x-2 overflow-clip rounded-sm p-1'
          >
            <File className='h-6 w-6 shrink-0 text-gray-500' />
            <span className='text-nowrap'>{object.split('/').at(-1)}</span>
          </div>
        ))}
      </div>
      <Separator orientation='vertical' />
    </div>
  );
}
