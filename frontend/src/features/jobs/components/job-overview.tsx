import { Card, KeyValueDisplay } from '@/components/common';
import { Link } from '@/components/common/link';
import { RelativeTime } from '@/components/common/relative-time';
import { Spinner } from '@/components/common/spinner/spinner';
import { Status, StatusProps } from '@/components/common/status';
import { Separator } from '@/components/ui/separator';
import { Routes } from '@/constants';
import { route } from '@/lib';
import { useGetQueue } from '@/services';
import { useGetClusterNode } from '@/services/clusters/get-cluster-node.query';
import { Job } from '@/types';

function getStatusType(jobStatus: Job['jobStatus']): StatusProps['status'] {
  switch (jobStatus) {
    case 'queued':
      return 'pending';
    case 'cancelled':
      return 'warning';
    case 'failed':
      return 'error';
    case 'running':
      return 'in-progress';
    case 'starting':
      return 'loading';
    case 'succeeded':
      return 'success';
  }
}

export interface JobOverviewProps {
  job: Job;
}

export function JobOverview(props: JobOverviewProps) {
  const { job } = props;
  const { data: queue, isLoading: queueLoading } = useGetQueue({ queueId: job.queueId });
  const { data: node, isLoading: nodeLoading } = useGetClusterNode({ nodeId: job.nodeId });

  return (
    <Card
      className='w-full'
      title={job.jobName}
      content={
        <div className='space-y-4'>
          <KeyValueDisplay
            items={[
              {
                key: 'ID',
                value: <span className='font-mono'>{job.jobId}</span>,
              },
              {
                key: 'Name',
                value: <span>{job.jobName}</span>,
              },
              {
                key: 'Status',
                value: <Status status={getStatusType(job.jobStatus)} className='capitalize'>{job.jobStatus}</Status>,
              },
              {
                key: 'Queue',
                value: queueLoading ? <Spinner className='m-1' size='xsmall' /> : <Link to={route(Routes.QUEUE_DETAILS, { queueId: job.queueId })}>{queue?.name}</Link>,
              },
              {
                key: 'Node',
                value: job.nodeId ? nodeLoading ? <Spinner className='m-1' size='xsmall' /> : <Link to={route(Routes.NODE_DETAILS, { nodeId: job.nodeId })}>{node?.id}</Link> : <span>&ndash;</span>,
              },
              {
                key: 'Submitted',
                value: <RelativeTime date={new Date(job.createdAt)} />,
              },
            ]}
          />
          <Separator />
          <p className='text-md font-medium '>Resource Requirements:</p>
          <KeyValueDisplay
            items={[
              {
                key: 'CPU',
                value: <div>{job.resourceRequirements.cpuMillicores}m</div>,
              },
              {
                key: 'Memory',
                value: <div>{job.resourceRequirements.memoryMb}MB</div>,
              },
              {
                key: 'GPUs',
                value: job.resourceRequirements.gpus ? <div>{job.resourceRequirements.gpus.count}x{job.resourceRequirements.gpus.model} ({job.resourceRequirements.gpus.memoryGb}GB</div> : <span>&ndash;</span>,
              },
            ]}
          />
        </div>
      }
    />
  );
}
