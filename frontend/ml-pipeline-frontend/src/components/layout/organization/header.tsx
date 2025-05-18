import OrganizationSelectionDropdown from '@/components/common/organization-selection-dropdown';
import { ThemeToggle } from '@/components/common/theme-toggle';
import { SidebarTrigger } from '@/components/ui/sidebar';

export default function Header() {
  return (
    <header className="bg-sidebar sticky top-0 z-50 flex w-full items-center border-b">
      <div className="flex h-(--header-height) w-full items-center justify-between gap-2 px-4">
        <div className="flex h-full flex-row items-center">
          <SidebarTrigger className="visible md:invisible" />
          <div className="flex h-full flex-row items-center">
            <OrganizationSelectionDropdown />
          </div>
        </div>
        <div>
          <ThemeToggle />
        </div>
      </div>
    </header>
  );
}
