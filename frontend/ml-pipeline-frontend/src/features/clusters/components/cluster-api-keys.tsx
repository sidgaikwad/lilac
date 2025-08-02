import { DataTable } from '@/components/common';
import { Card } from '@/components/common/card';
import { toast } from '@/components/toast';
import { Button } from '@/components/ui/button';
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
} from '@/components/ui/dialog';
import { Input } from '@/components/ui/input';
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip';
import { getFormattedTime, getRelativeTime } from '@/lib';
import { NewApiKey } from '@/model/api-key';
import {
  useCreateClusterKey,
  useDeleteClusterKey,
  useListClusterKeys,
} from '@/services';
import { ClusterApiKey } from '@/types';
import { ColumnDef, createColumnHelper } from '@tanstack/react-table';
import { Trash2 } from 'lucide-react';
import { useState } from 'react';

export interface ClusterApiKeyProps {
  clusterId?: string;
}

export function ClusterApiKeys(props: ClusterApiKeyProps) {
  const [newApiKey, setNewApiKey] = useState<NewApiKey | null>(null);
  const [isDialogOpen, setDialogOpen] = useState(false);

  const { data: keys, isLoading } = useListClusterKeys({
    clusterId: props.clusterId,
    onError: (error) =>
      toast.error('Error', {
        description: error.error,
      }),
  });

  const { mutate: createApiKey, isPending: createIsPending } =
    useCreateClusterKey({
      onSuccess: (data) => {
        setNewApiKey(data);
        setDialogOpen(true);
      },
    });
  const { mutate: deleteApiKey } = useDeleteClusterKey();

  const columnHelper = createColumnHelper<ClusterApiKey>();
  const columns: ColumnDef<ClusterApiKey>[] = [
    columnHelper.accessor('id', {
      header: 'Key ID',
      cell: ({ cell }) => {
        return <div className='font-mono'>{cell.renderValue()}</div>;
      },
    }),
    columnHelper.accessor('prefix', {
      header: 'Prefix',
    }),
    columnHelper.accessor('createdAt', {
      header: 'Created',
      cell: ({ cell }) => {
        const date = new Date(cell.getValue());
        return (
          <Tooltip>
            <TooltipTrigger>
              <p className='underline decoration-dotted'>{getRelativeTime(date)}</p>
            </TooltipTrigger>
            <TooltipContent side='bottom'>
              <p>{getFormattedTime(date)}</p>
            </TooltipContent>
          </Tooltip>
        );
        return <div>{getRelativeTime(date)}</div>;
      },
    }),
    columnHelper.display({
      id: 'actions',
      header: 'Actions',
      cell: ({ row }) => {
        return (
          <Trash2
            className='text-red-600 hover:cursor-pointer hover:text-red-700'
            onClick={() =>
              deleteApiKey({
                clusterId: row.original.clusterId,
                keyId: row.original.id,
              })
            }
          />
        );
      },
    }),
  ] as Array<ColumnDef<ClusterApiKey>>;

  return (
    <div>
      <Card
        className='w-full'
        title='API Keys'
        action={
          <Button
            onClick={() =>
              props.clusterId !== undefined &&
              createApiKey({ clusterId: props.clusterId })
            }
            disabled={createIsPending}
          >
            {createIsPending ? 'Generating...' : 'Generate New Key'}
          </Button>
        }
        content={
          <DataTable
            getRowId={(row) => row.id}
            columns={columns}
            data={keys ?? []}
            isLoading={isLoading}
          />
        }
      />
      <Dialog open={isDialogOpen} onOpenChange={setDialogOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>New API Key Generated</DialogTitle>
            <DialogDescription>
              Please copy your new API key. You will not be able to see it
              again.
            </DialogDescription>
          </DialogHeader>
          <div className='mt-4'>
            <Input readOnly value={newApiKey?.key} />
          </div>
        </DialogContent>
      </Dialog>
    </div>
  );
}
