import { useListOrganizations } from '@/services';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { Button } from '@/components/ui/button';
import { Skeleton } from '@/components/ui/skeleton';
import { ChevronDown, PlusIcon } from 'lucide-react';
import useOrganizationStore from '@/store/useOrganizationStore';
import { shallow } from 'zustand/shallow';

export default function OrganizationSelectionDropdown() {
  const { data: organizations, isLoading } = useListOrganizations();
  const { selectedOrganizationId, setSelectedOrganizationId } =
    useOrganizationStore((state) => ({
      selectedOrganizationId: state.selectedOrganizationId,
      setSelectedOrganizationId: state.setSelectedOrganizationId,
    }), shallow);

  return (
    <div className='flex flex-1'>
      {!organizations || isLoading ? (
        <Skeleton className="w-24 h-6 bg-muted" />
      ) : (
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button
              variant="ghost"
              size="sm"
              className="flex items-center gap-1 px-2 h-7 text-xs"
            >
              <span className="truncate max-w-[100px]">
                {organizations.find((org) => org.id === selectedOrganizationId)
                  ?.name ?? 'Select Org'}
              </span>
              <ChevronDown className="h-4 w-4 text-muted-foreground ml-1" />
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="start">
            {organizations.length > 0 ? (
              organizations.map((org) => (
                <DropdownMenuItem
                  key={org.id}
                  onSelect={() => setSelectedOrganizationId(org.id)}
                >
                  {org.name}
                </DropdownMenuItem>
              ))
            ) : (
              <DropdownMenuItem disabled>
                No organizations found
              </DropdownMenuItem>
            )}
            <DropdownMenuSeparator />
            <DropdownMenuItem disabled>
              <PlusIcon /> Create organization
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      )}
    </div>
  );
}
