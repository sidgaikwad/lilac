import { DataTable } from '@/components/common';
import { Button } from '@/components/ui/button';
import { Card } from '@/components/common/card';
import { Status, StatusProps } from '@/components/common/status';
import { toast } from '@/components/toast';
import { useCancelTrainingJob } from '@/services/training-jobs/cancel-training-job.mutation';
import { useListClusterJobs } from '@/services';
import { ClusterJob } from '@/types';
import { ColumnDef } from '@tanstack/react-table';
import { startCase } from 'lodash';

const CancelJobButton = ({
  jobId,
  status,
}: {
  jobId: string;
  status: string;
}) => {
  const { mutate, isPending } = useCancelTrainingJob({
    onSuccess: () => {
      toast.success('Job cancelled successfully');
    },
  });

  const isTerminal = ['succeeded', 'failed', 'cancelled'].includes(
    status.toLowerCase()
  );

  return (
    <Button
      variant='destructive'
      size='sm'
      onClick={() => mutate({ jobId })}
      disabled={isPending || isTerminal}
    >
      Cancel
    </Button>
  );
};

export const JOB_COLUMNS: ColumnDef<ClusterJob>[] = [
  {
    accessorKey: 'jobId',
    header: 'Job ID',
    cell: ({ cell }) => {
      return <div className='font-mono'>{cell.renderValue() as string}</div>;
    },
  },
  {
    accessorKey: 'jobName',
    header: 'Name',
  },
  {
    accessorKey: 'jobStatus',
    header: 'Status',
    cell: ({ cell }) => {
      const value = cell.renderValue() as string;
      let status: StatusProps['status'] = 'info';
      switch (value) {
        case 'queued':
          status = 'pending';
          break;
        case 'running':
          status = 'in-progress';
          break;
        case 'succeeded':
          status = 'success';
          break;
        case 'failed':
          status = 'error';
          break;
        case 'cancelled':
          status = 'warning';
          break;
      }
      return (
        <Status status={status}>
          {startCase(cell.renderValue() as string)}
        </Status>
      );
    },
  },
  {
    accessorKey: 'nodeId',
    header: 'Assigned Node ID',
    cell: ({ cell }) => {
      return <div className='font-mono'>{cell.renderValue() as string}</div>;
    },
  },
  {
    accessorKey: 'queueId',
    header: 'Queue ID',
    cell: ({ cell }) => {
      return <div className='font-mono'>{cell.renderValue() as string}</div>;
    },
  },
  {
    accessorKey: 'createdAt',
    header: 'Submitted At',
    cell: ({ cell }) => {
      const date = new Date(cell.renderValue() as string);
      return <div>{date.toLocaleString()}</div>;
    },
  },
  {
    id: 'actions',
    cell: ({ row }) => {
      return (
        <CancelJobButton
          jobId={row.original.jobId}
          status={row.original.jobStatus}
        />
      );
    },
  },
];

export interface ClusterJobsProps {
  clusterId?: string;
}

export function ClusterJobs(props: ClusterJobsProps) {
  const { data: jobs, isLoading } = useListClusterJobs({
    clusterId: props.clusterId,
    onError: (error) =>
      toast.error('Error', {
        description: error.error,
      }),
  });
  return (
    <Card
      className='w-full'
      title='Jobs'
      content={
        <DataTable
          columns={JOB_COLUMNS}
          data={jobs ?? []}
          isLoading={isLoading}
        />
      }
    />
  );
}
