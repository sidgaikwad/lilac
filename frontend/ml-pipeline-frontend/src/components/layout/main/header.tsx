import OrgaztionSelectionDropdown from '@/components/common/organization-selection-dropdown';
import ProjectSelectionDropdown from '@/components/common/project-selection-dropdown';
import { Separator } from '@/components/ui/separator';
import { SidebarTrigger } from '@/components/ui/sidebar';

export default function Header() {
  return (
    <header className="flex sticky top-0 z-50 w-full items-center border-b bg-sidebar">
      <div className="flex h-(--header-height) w-full items-center gap-2 px-4">
        <SidebarTrigger className='visible md:invisible' />
        <div className="flex flex-row items-center h-full">
          <OrgaztionSelectionDropdown />
          <Separator orientation='vertical' className='mr-2 data-[orientation=vertical]:h-[40%]'/>
          <ProjectSelectionDropdown />
        </div>
      </div>
    </header>
  );
}
