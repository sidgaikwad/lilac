import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
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
import { useDeleteQueue, useListClusters, useListQueues } from '@/services';
import { Link } from '@/components/common/link';

export function QueuesCard() {
  const { data: queues, isLoading } = useListQueues();
  const deleteQueueMutation = useDeleteQueue({});
  const { data: clusters } = useListClusters();

  const clustersMap: Record<string, string> = (clusters ?? []).reduce(
    (obj, cluster) => {
      return {
        [cluster.clusterId]: cluster.clusterName,
        ...obj,
      };
    },
    {}
  );

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
                  <TableCell>
                    <Link to={`/queues/${queue.id}`}>{queue.name}</Link>
                  </TableCell>
                  <TableCell>{queue.priority}</TableCell>
                  <TableCell>
                    {queue.clusterTargets.map((id, index) => {
                      return (
                        <span>
                          <Link to={`/clusters/${id}`}>{clustersMap[id]}</Link>
                          {index !== queue.clusterTargets.length - 1 && <>,&nbsp;</>}
                        </span>
                      );
                    })}
                  </TableCell>
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
