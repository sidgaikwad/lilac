import { Avatar, AvatarFallback } from '@/components/ui/avatar';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { LogoutButton } from '@/components/common/LogoutButton';
import { User } from '@/types';
import { Skeleton } from '@/components/ui/skeleton';

interface UserProfileDropdownProps {
  user: User | null | undefined;
  isLoading?: boolean;
}

export function UserProfileDropdown({ user, isLoading }: UserProfileDropdownProps) {
  const getInitials = (email?: string) => {
    if (!email) return 'U';
    return email.charAt(0).toUpperCase();
  };

  if (isLoading) {
    return <Skeleton className="h-8 w-8 rounded-full" />;
  }

  if (!user) {
    return null; 
  }

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <button className="focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 rounded-full">
          <Avatar>
            {/* <AvatarImage src={user.avatarUrl} alt={user.name || user.email} /> */}
            <AvatarFallback>{getInitials(user.email)}</AvatarFallback>
          </Avatar>
        </button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end" className="w-56">
        <DropdownMenuLabel className="truncate font-normal">
          <div className="flex flex-col space-y-1">
            <p className="text-sm font-medium leading-none">Signed in as</p>
            <p className="text-xs leading-none text-muted-foreground truncate">
              {user.email}
            </p>
          </div>
        </DropdownMenuLabel>
        <DropdownMenuSeparator />
        <DropdownMenuItem asChild className="p-0 cursor-pointer">
          <LogoutButton className="w-full justify-start h-8" />
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}