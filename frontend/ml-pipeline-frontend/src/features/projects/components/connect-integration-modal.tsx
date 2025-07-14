import {
  Dialog,
  DialogTrigger,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
  DialogClose,
  DialogDescription,
} from '@/components/ui/dialog';
import { Button } from '@/components/ui/button';
import { AwsLogo } from '@/icons/aws';
import { ConnectAwsIntegrationForm } from '../forms/connect-aws-integration';
import { Project } from '@/types';
import { useState } from 'react';
import {
  Card,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';

export interface ConnectIntegrationModalProps {
  isOpen: boolean;
  setOpen: (input: boolean) => void;
  project: Project;
  showTrigger?: boolean;
}

export function ConnectIntegrationModal({
  isOpen,
  setOpen,
  project,
  showTrigger = true,
}: ConnectIntegrationModalProps) {
  const [selectedIntegration, setSelectedIntegration] = useState<string>();
  const handleOpenChange = (openState: boolean) => {
    setOpen(openState);
  };

  const getFormContent = () => {
    switch (selectedIntegration) {
      case 'aws':
        return (
          <ConnectAwsIntegrationForm
            projectId={project.id}
            onSubmit={() => {
              setSelectedIntegration(undefined);
            }}
            onCancel={() => {
              setSelectedIntegration(undefined);
            }}
          />
        );
      default:
        return (
          <div>
            <Card
              data-disabled={project.awsIntegration !== undefined}
              className='max-h-xs data-[disabled=false]:hover:bg-muted data-[disabled=true]:bg-muted data-[disabled=true]:text-gray-text-muted max-w-xs data-[disabled=false]:hover:cursor-pointer'
              onClick={() =>
                project.awsIntegration === undefined &&
                setSelectedIntegration('aws')
              }
            >
              <CardHeader>
                <div className='flex flex-row'>
                  <AwsLogo className='h-full max-h-16 w-full max-w-16 self-center pr-2' />
                  <div className='flex h-full flex-col border-l pl-2'>
                    <CardTitle>AWS</CardTitle>
                    <CardDescription>
                      Configure access to your AWS account
                    </CardDescription>
                  </div>
                </div>
              </CardHeader>
            </Card>
          </div>
        );
    }
  };

  return (
    <Dialog open={isOpen} onOpenChange={handleOpenChange}>
      {showTrigger && (
        <DialogTrigger asChild>
          <Button variant='default'>Connect services</Button>
        </DialogTrigger>
      )}
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Connect Services</DialogTitle>
          <DialogDescription></DialogDescription>
        </DialogHeader>
        {getFormContent()}
        <DialogFooter>
          <DialogClose></DialogClose>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}

export default ConnectIntegrationModal;
