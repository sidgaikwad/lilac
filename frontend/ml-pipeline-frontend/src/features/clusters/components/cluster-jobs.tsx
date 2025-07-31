import { DataTable } from '@/components/common';
import { Card } from '@/components/common/card';
import { Status, StatusProps } from '@/components/common/status';
import { toast } from '@/components/toast';
import { useGetClusterJobs } from '@/services/clusters/get-cluster-jobs.query';
import { ClusterJob } from '@/types';
import { ColumnDef } from '@tanstack/react-table';
import { startCase } from 'lodash';

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
];

export interface ClusterJobsProps {
  clusterId?: string;
}

export function ClusterJobs(props: ClusterJobsProps) {
  const { data: jobs, isLoading } = useGetClusterJobs({
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
