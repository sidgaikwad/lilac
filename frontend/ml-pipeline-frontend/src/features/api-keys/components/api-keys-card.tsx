import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import { useListApiKeys } from '@/services/api-keys/list-api-keys.query';
import { useCreateApiKey } from '@/services/api-keys/create-api-key.mutation';
import { useDeleteApiKey } from '@/services/api-keys/delete-api-key.mutation';
import { Button } from '@/components/ui/button';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import { useState } from 'react';
import { NewApiKey } from '@/model/api-key';
import { Input } from '@/components/ui/input';

export function ApiKeysCard() {
  const [newApiKey, setNewApiKey] = useState<NewApiKey | null>(null);
  const [isDialogOpen, setDialogOpen] = useState(false);

  const { data: apiKeys, isLoading } = useListApiKeys();
  const createApiKeyMutation = useCreateApiKey({
    onSuccess: (data) => {
      setNewApiKey(data);
      setDialogOpen(true);
    },
  });
  const deleteApiKeyMutation = useDeleteApiKey({});

  const handleGenerateKey = () => {
    createApiKeyMutation.mutate();
  };

  return (
    <>
      <Card>
        <CardHeader>
          <div className="flex justify-between items-center">
            <div>
              <CardTitle>API Keys</CardTitle>
              <CardDescription>
                Your API keys are listed below.
              </CardDescription>
            </div>
            <Button
              onClick={handleGenerateKey}
              disabled={createApiKeyMutation.isPending}
            >
              {createApiKeyMutation.isPending
                ? 'Generating...'
                : 'Generate New Key'}
            </Button>
          </div>
        </CardHeader>
        <CardContent>
          {isLoading ? (
            <p>Loading...</p>
          ) : (
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>Prefix</TableHead>
                  <TableHead>Created At</TableHead>
                  <TableHead>Expires At</TableHead>
                  <TableHead>Last Used</TableHead>
                  <TableHead></TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {apiKeys?.map((key) => (
                  <TableRow key={key.id}>
                    <TableCell>{key.prefix}</TableCell>
                    <TableCell>
                      {new Date(key.created_at).toLocaleString()}
                    </TableCell>
                    <TableCell>
                      {key.expires_at
                        ? new Date(key.expires_at).toLocaleString()
                        : 'Never'}
                    </TableCell>
                    <TableCell>
                      {key.last_used_at
                        ? new Date(key.last_used_at).toLocaleString()
                        : 'Never'}
                    </TableCell>
                    <TableCell>
                      <Button
                        variant="destructive"
                        size="sm"
                        onClick={() =>
                          deleteApiKeyMutation.mutate({ keyId: key.id })
                        }
                        disabled={deleteApiKeyMutation.isPending}
                      >
                        Delete
                      </Button>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
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
          <div className="mt-4">
            <Input readOnly value={newApiKey?.key} />
          </div>
        </DialogContent>
      </Dialog>
    </>
  );
}