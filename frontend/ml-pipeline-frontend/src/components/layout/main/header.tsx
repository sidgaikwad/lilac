import { ThemeToggle } from '@/components/common/theme-toggle';
import { UserProfileDropdown } from '@/components/common/user-profile-dropdown';
import { useGetAccountDetails } from '@/services/account/get-account-details.query';
import useAuthStore from '@/store/use-auth-store';
import { Skeleton } from '@/components/ui/skeleton';
import { SidebarTrigger } from '@/components/ui/sidebar';

export default function Header() {
  const token = useAuthStore((state) => state.token);
  const { data: user, isLoading } = useGetAccountDetails({ enabled: !!token });

  return (
    <header className='bg-accent-background-1 border-gray-border-subtle sticky top-0 z-50 flex w-full items-center border-b'>
      <div className='flex h-(--header-height) w-full items-center justify-between gap-2 px-4'>
        <div className='flex h-full flex-row items-center'>
          <div className='flex h-full flex-row items-center'>
            <SidebarTrigger className='visible md:invisible' />
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
