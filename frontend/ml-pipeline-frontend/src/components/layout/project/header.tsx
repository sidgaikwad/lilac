import { ThemeToggle } from '@/components/common/theme-toggle';
import { UserProfileDropdown } from '@/components/common/user-profile-dropdown';
import { SidebarTrigger } from '@/components/ui/sidebar';
import { useGetAccountDetails } from '@/services/auth/get-account-details.query';
import useAuthStore from '@/store/use-auth-store';
import { Skeleton } from '@/components/ui/skeleton';
import ProjectSelectionDropdown from '@/components/common/project-selection-dropdown';
import { Shapes } from 'lucide-react';
import { Link } from 'react-router-dom';

export default function Header() {
  const token = useAuthStore((state) => state.token);
  const { data: user, isLoading } = useGetAccountDetails({ enabled: !!token });

  return (
    <header className='bg-accent-background-1 border-gray-border-subtle sticky top-0 z-50 flex w-full items-center border-b'>
      <div className='flex h-(--header-height) w-full items-center justify-between gap-2 px-4'>
        <div className='flex h-full flex-row items-center space-x-8'>
          <SidebarTrigger className='block sm:hidden' />
          <Link to='/' className='h-full'>
            <img src='/lilac-header.png' className='h-full p-2 dark:hidden' />
            <img
              src='/lilac-header-white.png'
              className='hidden h-full p-2 dark:block'
            />
          </Link>
          <div className='flex flex-row items-center'>
            <Shapes className='text-gray-text pr-1' />
            <div className='bg-accent-secondary border-accent-border w-fit rounded-md border px-2'>
              <ProjectSelectionDropdown />
            </div>
          </div>
        </div>
        <div className='flex items-center gap-2'>
          <ThemeToggle />
          {token && isLoading && <Skeleton className='h-8 w-8 rounded-full' />}
          {token && !isLoading && user && <UserProfileDropdown user={user} />}
        </div>
      </div>
    </header>
  );
}
