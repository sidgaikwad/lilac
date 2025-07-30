import {
  Container,
  ContainerContent,
  ContainerDescription,
  ContainerHeader,
  ContainerTitle,
} from '@/components/ui/container';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { ApiKeysCard } from '../components/api-keys-card';

export function ApiKeysSettings() {
  return (
    <Container className='h-full w-full'>
      <ContainerHeader>
        <ContainerTitle>API Keys</ContainerTitle>
        <ContainerDescription>
          Manage your API keys.
        </ContainerDescription>
      </ContainerHeader>
      <ContainerContent>
        <Tabs defaultValue='api-keys' className='w-full'>
          <TabsList className='max-w-[400px]'>
            <TabsTrigger value='api-keys'>API Keys</TabsTrigger>
          </TabsList>
          <TabsContent value='api-keys'>
            <ApiKeysCard />
          </TabsContent>
        </Tabs>
      </ContainerContent>
    </Container>
  );
}