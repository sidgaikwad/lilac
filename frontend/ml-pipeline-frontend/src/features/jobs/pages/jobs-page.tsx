import { useCancelTrainingJob, useListJobs } from '@/services';
import {
  Container,
  ContainerAction,
  ContainerContent,
  ContainerDescription,
  ContainerHeader,
  ContainerTitle,
} from '@/components/ui/container';
import Breadcrumbs from '@/components/common/breadcrumbs';
import { toast } from '@/components/toast';
import { Card, DataTable } from '@/components/common';
import { ColumnDef } from '@tanstack/react-table';
import { Job } from '@/types/api/job';
import { Status, StatusProps } from '@/components/common/status';
import { startCase } from 'lodash';
import { Button } from '@/components/ui/button';
import { RelativeTime } from '@/components/common/relative-time';
import { Link } from '@/components/common/link';
import { route } from '@/lib';
import { Routes } from '@/constants';


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

const JOB_COLUMNS: ColumnDef<Job>[] = [
  {
    accessorKey: 'jobId',
    header: 'Job ID',
    cell: ({ cell }) => {
      return <Link className='font-mono overflow-ellipsis' to={`/jobs/${cell.getValue() as string}`}>{cell.renderValue() as string}</Link>;
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
      return <Link to={route(Routes.QUEUE_DETAILS, { queueId: cell.getValue() as string })} className='font-mono'>{cell.renderValue() as string}</Link>;
    },
  },
  {
    accessorKey: 'createdAt',
    header: 'Submitted',
    cell: ({ cell }) => {
      const date = new Date(cell.renderValue() as string);
      return <RelativeTime date={date} />;
    },
  },
  {
    id: 'actions',
    header: 'Actions',
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


function JobsPage() {
  const { data: jobs, isLoading } = useListJobs({
    onError: (error) =>
      toast.error('Failed to load datasets', {
        description: error.error,
      }),
  });

  return (
    <Container>
      <ContainerHeader>
        <div className='flex-1 shrink-0 grow-0 basis-full pb-4'>
          <Breadcrumbs
            breadcrumbs={[
              {
                content: 'Jobs',
                link: `/`,
              },
            ]}
          />
        </div>
        <ContainerTitle>
          Jobs
          <ContainerDescription></ContainerDescription>
        </ContainerTitle>
        <ContainerAction></ContainerAction>
      </ContainerHeader>

      <ContainerContent>
        <div className='flex flex-row space-x-4'>
          <Card
            className='w-full'
            content={
              <DataTable
                columns={JOB_COLUMNS}
                data={jobs ?? []}
                isLoading={isLoading}
              />
            }
          />
        </div>
      </ContainerContent>
    </Container>
  );
}

export default JobsPage;
