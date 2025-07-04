import { Badge } from '@/components/ui/badge';
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
  CardFooter,
  CardAction,
} from '@/components/ui/card';
import {
  Table,
  TableBody,
  TableCaption,
  TableCell,
  TableFooter,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import { Project } from '@/types';
import ConnectIntegrationModal from './connect-integration-modal';
import { useState } from 'react';
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '@/components/ui/popover';
import { Label } from '@/components/ui/label';

export interface IntegrationSettingsProps {
  project: Project;
}

export function IntegrationSettings(props: IntegrationSettingsProps) {
  const { project } = props;

  const [isIntegrationModalOpen, setIntegrationModalOpen] = useState(false);

  return (
    <Card>
      <CardHeader>
        <CardTitle>Integrations</CardTitle>
        <CardDescription>
          Set up integrations with third party services
        </CardDescription>
        <CardAction>
          <ConnectIntegrationModal
            project={project}
            isOpen={isIntegrationModalOpen}
            setOpen={setIntegrationModalOpen}
          />
        </CardAction>
      </CardHeader>
      <CardContent>
        <Table>
          <TableCaption>
            {project.awsIntegration
              ? 'Configured integrations'
              : 'No configured integrations'}
          </TableCaption>
          <TableHeader>
            <TableRow>
              <TableHead>Integration</TableHead>
              <TableHead>Status</TableHead>
              <TableHead className='col-span-3'>Metadata</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {project.awsIntegration && (
              <TableRow key='aws'>
                <TableCell className='font-medium'>AWS</TableCell>
                <TableCell>
                  <Badge className='bg-green-700'>Connected</Badge>
                </TableCell>
                <TableCell>
                  <Popover>
                    <PopoverTrigger>
                      <span className='rounded-full bg-gray-300 px-3 py-1 underline decoration-dotted hover:bg-gray-400/50'>
                        Show metadata
                      </span>
                    </PopoverTrigger>
                    <PopoverContent className='w-fit'>
                      <div className='grid grid-cols-4 space-y-2'>
                        <Label>Role ARN</Label>
                        <code className='col-span-3'>
                          {project.awsIntegration.roleArn}
                        </code>
                        <Label>External ID</Label>
                        <code className='col-span-3'>
                          {project.awsIntegration.externalId}
                        </code>
                      </div>
                    </PopoverContent>
                  </Popover>
                </TableCell>
              </TableRow>
            )}
          </TableBody>
          <TableFooter></TableFooter>
        </Table>
      </CardContent>
      <CardFooter></CardFooter>
    </Card>
  );
}
