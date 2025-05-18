import { useState, useRef } from 'react';
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
import useOrganizationStore from '@/store/use-organization-store';
import { shallow } from 'zustand/shallow';
import { generatePath, useNavigate } from 'react-router-dom';
import { Routes } from '@/constants';
import CreateOrganizationModal from '@/features/organizations/components/create-organization-modal';

export default function OrganizationSelectionDropdown() {
  const navigate = useNavigate();
  const { data: organizations, isLoading } = useListOrganizations();
  const { selectedOrganizationId, setSelectedOrganizationId } =
    useOrganizationStore(
      (state) => ({
        selectedOrganizationId: state.selectedOrganizationId,
        setSelectedOrganizationId: state.setSelectedOrganizationId,
      }),
      shallow
    );
  
  const [isCreateOrgModalOpen, setCreateOrgModalOpen] = useState(false);
  const triggerRef = useRef<HTMLButtonElement>(null);

  const handleModalOpenChange = (open: boolean) => {
    setCreateOrgModalOpen(open);
    if (!open && triggerRef.current) {
      setTimeout(() => triggerRef.current?.focus(), 0);
    }
  };

  return (
    <>
      <div className="flex flex-1">
        {!organizations || isLoading ? (
          <Skeleton className="bg-muted h-6 w-24" />
        ) : (
          <DropdownMenu modal={false}>
            <DropdownMenuTrigger asChild>
              <Button
                ref={triggerRef}
                variant="ghost"
                size="sm"
                className="flex h-7 items-center gap-1 px-2 text-xs"
              >
                <span className="max-w-[100px] truncate">
                  {organizations.find((org) => org.id === selectedOrganizationId)
                    ?.name ?? 'Select Org'}
                </span>
                <ChevronDown className="text-muted-foreground ml-1 h-4 w-4" />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="start">
              {organizations.length > 0 ? (
                organizations.map((org) => (
                  <DropdownMenuItem
                    key={org.id}
                    onSelect={() => {
                      setSelectedOrganizationId(org.id);
                      navigate(
                        generatePath(Routes.ORGANIZATION_PROJECTS, {
                          organizationId: org.id,
                        })
                      );
                    }}
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
              <DropdownMenuItem onSelect={() => setCreateOrgModalOpen(true)}>
                <PlusIcon className="mr-2 h-4 w-4" />
                <span>Create organization</span>
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        )}
      </div>
      <CreateOrganizationModal
        isOpen={isCreateOrgModalOpen}
        setOpen={handleModalOpenChange}
        showTrigger={false}
      />
    </>
  );
}
