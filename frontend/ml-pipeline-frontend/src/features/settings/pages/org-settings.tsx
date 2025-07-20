import {
  Container,
  ContainerContent,
  ContainerDescription,
  ContainerHeader,
  ContainerTitle,
} from '@/components/ui/container';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { CredentialsSettingsCard } from '../components/credentials-settings-card';

export function OrgSettings() {
  return (
    <Container className='h-full w-full'>
      <ContainerHeader>
        <ContainerTitle>Settings</ContainerTitle>
        <ContainerDescription>
          View and manage your settings.
        </ContainerDescription>
      </ContainerHeader>
      <ContainerContent>
        <Tabs defaultValue='credentials' className='w-full'>
          <TabsList className='max-w-[400px]'>
            {/* <TabsTrigger value='general'>General</TabsTrigger> */}
            <TabsTrigger value='credentials'>Credentials</TabsTrigger>
          </TabsList>
          {/* <TabsContent value='general'>
            Make changes to your account here.
          </TabsContent> */}
          <TabsContent value='credentials'>
            <CredentialsSettingsCard />
          </TabsContent>
        </Tabs>
      </ContainerContent>
    </Container>
  );
}
