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
        <Tabs defaultValue='general'>
          <TabsList>
            <TabsTrigger value='general'>General</TabsTrigger>
            <TabsTrigger value='integrations'>Integrations</TabsTrigger>
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
