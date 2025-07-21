import { Card } from '@/components/common/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Link } from 'react-router-dom';
import { Workspace } from '@/types/api/workspace';
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
  projectId: string;
}

const statusVariant: Record<
  Workspace['status'],
  'default' | 'destructive' | 'secondary' | 'outline'
> = {
  running: 'default',
  stopped: 'secondary',
  pending: 'outline',
  failed: 'destructive',
  stopping: 'secondary',
  terminated: 'destructive',
};

function WorkspaceListItem({
  workspace,
  onStart,
  onStop,
  projectId,
}: WorkspaceListItemProps) {
  const { id, name, status, ide, cpu_millicores, memory_mb } = workspace;

  const [isEditModalOpen, setEditModalOpen] = useState(false);

  return (
    <>
      <Card
        className={cn(
          'max-w-sm w-full h-full flex flex-col gap-4 p-4',
          status.toLowerCase() === 'running'
            ? 'border-accent-border hover:border-accent-border-hover'
            : ''
        )}
        title={name}
        description={
          <div className="flex flex-col gap-1 text-sm text-muted-foreground">
            <span>{ide}</span>
            <span>
              {cpu_millicores / 1000} vCPUs, {memory_mb / 1024} GB RAM
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
                {status.toLowerCase() === 'running' ? (
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
                      className='text-destructive'
                      onSelect={(e) => e.preventDefault()}
                    >
                      Delete
                    </DropdownMenuItem>
                  }
                  onConfirm={() => console.log('TODO: Delete workspace', id)}
                  itemName={name}
                  itemType='workspace'
                />
              </DropdownMenuContent>
            </DropdownMenu>
          </div>
        }
        footer={
          status.toLowerCase() === 'running' ? (
            <Button asChild>
              <Link to={`/projects/${projectId}/workspaces/${id}`}>Connect</Link>
            </Button>
          ) : (
            <div />
          )
        }
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
