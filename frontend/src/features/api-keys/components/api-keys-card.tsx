import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import { Button } from '@/components/ui/button';

import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import { useState } from 'react';
import { Input } from '@/components/ui/input';
import { Trash2 } from 'lucide-react';
import { ColumnDef, createColumnHelper } from '@tanstack/react-table';
import { ApiKey, NewApiKey } from '@/types';
import { RelativeTime } from '@/components/common/relative-time';
import { useCreateApiKey, useDeleteApiKey, useListApiKeys } from '@/services';
import { DataTable } from '@/components/common';

export function ApiKeysCard() {
  const [newApiKey, setNewApiKey] = useState<NewApiKey | null>(null);
  const [isDialogOpen, setDialogOpen] = useState(false);

  const { data: apiKeys, isLoading } = useListApiKeys();
  const { mutate: createApiKey, isPending } = useCreateApiKey({
    onSuccess: (data) => {
      setNewApiKey(data);
      setDialogOpen(true);
    },
  });
  const { mutate: deleteApiKey } = useDeleteApiKey({});

  const columnHelper = createColumnHelper<ApiKey>();
  const columns: ColumnDef<ApiKey>[] = [
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
        return <RelativeTime date={date} />;
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
                keyId: row.original.id,
              })
            }
          />
        );
      },
    }),
  ] as Array<ColumnDef<ApiKey>>;

  return (
    <>
      <Card>
        <CardHeader>
          <div className='flex items-center justify-between'>
            <div>
              <CardTitle>API Keys</CardTitle>
              <CardDescription>Your API keys are listed below.</CardDescription>
            </div>
            <Button onClick={() => createApiKey()} disabled={isPending}>
              {isPending ? 'Generating...' : 'Generate New Key'}
            </Button>
          </div>
        </CardHeader>
        <CardContent>
          {isLoading ? (
            <p>Loading...</p>
          ) : (
            <DataTable columns={columns} data={apiKeys ?? []} />
          )}
        </CardContent>
      </Card>
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
    </>
  );
}
