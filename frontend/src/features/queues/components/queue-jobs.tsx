import { DataTable } from '@/components/common';
import { Card } from '@/components/common/card';
import { Link } from '@/components/common/link';
import { Status, StatusProps } from '@/components/common/status';
import { toast } from '@/components/toast';
import { Input } from '@/components/ui/input';
import { Routes } from '@/constants';
import { route } from '@/lib';
import { useListQueueJobs } from '@/services';
import { QueueJob } from '@/types';
import { ColumnDef } from '@tanstack/react-table';
import { startCase } from 'lodash';

const JOB_COLUMNS: ColumnDef<QueueJob>[] = [
  {
    accessorKey: 'jobId',
    header: 'Job ID',
    cell: ({ cell }) => {
      return (
        <Link
          to={route(Routes.JOB_DETAILS, { jobId: cell.getValue() as string })}
          className='font-mono'
        >
          {cell.renderValue() as string}
        </Link>
      );
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
    accessorKey: 'createdAt',
    header: 'Submitted At',
    cell: ({ cell }) => {
      const date = new Date(cell.renderValue() as string);
      return <div>{date.toLocaleString()}</div>;
    },
  },
];

export interface QueueJobsProps {
  queueId?: string;
}

export function QueueJobs(props: QueueJobsProps) {
  const { data: jobs, isLoading } = useListQueueJobs({
    queueId: props.queueId,
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
          renderFilters={(table) => {
            return (
              <Input
                className='w-full min-w-sm'
                placeholder='Filter jobs by name...'
                value={
                  (table.getColumn('jobName')?.getFilterValue() as string) ?? ''
                }
                onChange={(event) =>
                  table.getColumn('jobName')?.setFilterValue(event.target.value)
                }
              />
            );
          }}
        />
      }
    />
  );
}
