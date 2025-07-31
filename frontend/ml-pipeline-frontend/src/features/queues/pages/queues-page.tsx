import {
  Container,
  ContainerContent,
  ContainerDescription,
  ContainerHeader,
  ContainerTitle,
} from '@/components/ui/container';
import { QueuesCard } from '../components/queues-card';

export function QueuesPage() {
  return (
    <Container className='h-full w-full'>
      <ContainerHeader>
        <ContainerTitle>Queues</ContainerTitle>
        <ContainerDescription>Manage your job queues.</ContainerDescription>
      </ContainerHeader>
      <ContainerContent>
        <QueuesCard />
      </ContainerContent>
    </Container>
  );
}
