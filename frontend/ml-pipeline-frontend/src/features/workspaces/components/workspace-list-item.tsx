import { Card } from '@/components/common/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Workspace } from '@/features/workspaces/mock-data';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { MoreHorizontal } from 'lucide-react';
import { cn } from '@/lib/utils';
import { EditWorkspaceModal } from './edit-workspace-modal';
import { useState } from 'react';
import DestructiveActionConfirmationModal from '@/components/common/destructive-action-confirmation-dialog';

export interface WorkspaceListItemProps {
  workspace: Workspace;
  onStart: (id: string) => void;
  onStop: (id: string) => void;
}

const statusVariant: Record<
  Workspace['status'],
  'default' | 'destructive' | 'secondary' | 'outline'
> = {
  Running: 'default',
  Stopped: 'secondary',
  Starting: 'outline',
  Error: 'destructive',
};

function WorkspaceListItem({
  workspace,
  onStart,
  onStop,
}: WorkspaceListItemProps) {
  const { id, name, status, environment, hardware, lastStarted } = workspace;

  const [isEditModalOpen, setEditModalOpen] = useState(false);

  return (
    <>
      <Card
        className={cn(
          'max-w-sm w-full h-full flex flex-col gap-4 p-4',
          status === 'Running'
            ? 'border-accent-border hover:border-accent-border-hover'
            : ''
        )}
        title={name}
      description={
        <div className='flex flex-col gap-2'>
          <span>
            {environment.name} on {hardware.tier}
          </span>
          <span className='text-xs'>
            Last started: {new Date(lastStarted).toLocaleString()}
          </span>
        </div>
      }
      action={
        <div className='flex items-center gap-2'>
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button variant='ghost' size='icon'>
                <MoreHorizontal className='h-4 w-4' />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent>
              {status === 'Running' ? (
                <DropdownMenuItem onClick={() => onStop(id)}>
                  Stop
                </DropdownMenuItem>
              ) : (
                <DropdownMenuItem onClick={() => onStart(id)}>
                  Start
                </DropdownMenuItem>
              )}
              <DropdownMenuItem onClick={() => setEditModalOpen(true)}>
                Settings
              </DropdownMenuItem>
              <DropdownMenuItem>Logs</DropdownMenuItem>
              <DestructiveActionConfirmationModal
                dialogTrigger={
                  <DropdownMenuItem
                    className="text-destructive"
                    onSelect={(e) => e.preventDefault()}
                  >
                    Delete
                  </DropdownMenuItem>
                }
                onConfirm={() => console.log('TODO: Delete workspace', id)}
                itemName={name}
                itemType="workspace"
              />
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
      }
      footer={<></>}
      footerAction={<Badge variant={statusVariant[status]}>{status}</Badge>}
    />
      <EditWorkspaceModal
        isOpen={isEditModalOpen}
        setOpen={setEditModalOpen}
        workspace={workspace}
      />
    </>
  );
}

export default WorkspaceListItem;