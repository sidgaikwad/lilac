import {
  Container,
  ContainerAction,
  ContainerContent,
  ContainerDescription,
  ContainerHeader,
  ContainerTitle,
} from '@/components/ui/container';
import Breadcrumbs from '@/components/common/breadcrumbs';
import { useLocation, useNavigate, useParams } from 'react-router-dom';
import { useSuspenseQuery } from '@tanstack/react-query';
import { useEffect } from 'react';
import { Tabs } from '@/components/common/tabs';
import { getJobQuery } from '@/services/training-jobs/get-job.query';
import { JobOverview } from '../components/job-overview';

function JobDetailsPage() {
  const location = useLocation();
  const navigate = useNavigate();
  const { jobId } = useParams<{
    jobId: string;
  }>();

  const { data: job } = useSuspenseQuery(getJobQuery(jobId));

  useEffect(() => {
    if (!location.hash) {
      navigate('#overview', {
        replace: true,
      });
    }
  }, [location.hash, navigate]);

  return (
    <Container>
      <ContainerHeader>
        <div className='flex-1 shrink-0 grow-0 basis-full pb-4'>
          <Breadcrumbs
            breadcrumbs={[
              {
                content: 'Jobs',
                link: '/jobs',
              },
              {
                content: job.jobName,
                link: `/jobs/${jobId}`,
              },
            ]}
          />
        </div>
        <ContainerTitle>
          {job.jobName}
          <ContainerDescription></ContainerDescription>
        </ContainerTitle>
        <ContainerAction></ContainerAction>
      </ContainerHeader>

      <ContainerContent>
        <Tabs
          defaultTab='overview'
          items={[
            {
              id: 'overview',
              content: <JobOverview job={job} />,
            },
          ]}
        />
      </ContainerContent>
    </Container>
  );
}

export default JobDetailsPage;
