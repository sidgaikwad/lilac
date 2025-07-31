import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import { useListQueues } from '@/services/queues/list-queues.query';
import { useDeleteQueue } from '@/services/queues/delete-queue.mutation';
import { Button } from '@/components/ui/button';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import { CreateQueueModal } from './create-queue-modal';

export function QueuesCard() {
  const { data: queues, isLoading } = useListQueues();
  const deleteQueueMutation = useDeleteQueue({});

  return (
    <Card>
      <CardHeader>
        <div className='flex items-center justify-between'>
          <div>
            <CardTitle>Queues</CardTitle>
            <CardDescription>Manage your job queues.</CardDescription>
          </div>
          <CreateQueueModal />
        </div>
      </CardHeader>
      <CardContent>
        {isLoading ? (
          <p>Loading...</p>
        ) : (
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Name</TableHead>
                <TableHead>Priority</TableHead>
                <TableHead>Cluster Targets</TableHead>
                <TableHead></TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {queues?.map((queue) => (
                <TableRow key={queue.id}>
                  <TableCell>{queue.name}</TableCell>
                  <TableCell>{queue.priority}</TableCell>
                  <TableCell>{queue.cluster_targets.join(', ')}</TableCell>
                  <TableCell>
                    <Button
                      variant='destructive'
                      size='sm'
                      onClick={() =>
                        deleteQueueMutation.mutate({ queueId: queue.id })
                      }
                      disabled={deleteQueueMutation.isPending}
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
  );
}
