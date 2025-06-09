import { getProjectQuery } from '@/services';
import { useParams } from 'react-router-dom';
import {
  Container,
  ContainerContent,
  ContainerDescription,
  ContainerHeader,
  ContainerTitle,
} from '@/components/ui/container';
import { useSuspenseQuery } from '@tanstack/react-query';
import { GeneralSettings } from '../components/general-settings';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { IntegrationSettings } from '../components/integration-settings';

function ProjectSettingsPage() {
  const { projectId } = useParams<'projectId'>();
  const { data: project } = useSuspenseQuery(getProjectQuery(projectId));

  return (
    <Container>
      <ContainerHeader>
        <ContainerTitle>Project Settings</ContainerTitle>
        <ContainerDescription>Configure your project</ContainerDescription>
      </ContainerHeader>
      <ContainerContent>
        <Tabs defaultValue='general' className='w-full'>
          <TabsList className='bg-background w-full justify-start rounded-none border-b p-0'>
            <TabsTrigger
              className='hover:text-primary hover:cursor-pointer bg-background data-[state=active]:border-b-primary h-full rounded-none border-b-2 data-[state=active]:shadow-none data-[state=active]:text-primary'
              value='general'
            >
              General
            </TabsTrigger>
            <TabsTrigger
              className='hover:text-primary hover:cursor-pointer bg-background data-[state=active]:border-b-primary h-full rounded-none border-b-2 data-[state=active]:shadow-none data-[state=active]:text-primary'
              value='integrations'
            >
              Integrations
            </TabsTrigger>
          </TabsList>
          <TabsContent value='general'>
            <GeneralSettings project={project} />
          </TabsContent>
          <TabsContent value='integrations'>
            <IntegrationSettings project={project} />
          </TabsContent>
        </Tabs>
      </ContainerContent>
    </Container>
  );
}

export default ProjectSettingsPage;
