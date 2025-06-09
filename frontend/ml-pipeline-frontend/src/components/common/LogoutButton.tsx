import * as React from 'react';
import { Button } from '@/components/ui/button';
import useAuthStore from '@/store/use-auth-store';
import { useNavigate } from 'react-router-dom';
import { LogOutIcon } from 'lucide-react';
import { Routes } from '@/constants';

interface LogoutButtonProps
  extends Omit<React.ComponentProps<typeof Button>, 'onClick'> {
  onClick?: (event: React.MouseEvent<HTMLButtonElement>) => void;
}

export function LogoutButton({
  onClick: parentOnClick,
  ...restProps
}: LogoutButtonProps) {
  const logoutAction = useAuthStore((state) => state.logout);
  const navigate = useNavigate();

  const handleLogoutClick = (event: React.MouseEvent<HTMLButtonElement>) => {
    console.log('LogoutButton clicked');

    if (parentOnClick) {
      parentOnClick(event);
    }

    logoutAction();
    navigate(Routes.LOGIN, { replace: true });
  };

  return (
    <Button variant='ghost' {...restProps} onClick={handleLogoutClick}>
      <LogOutIcon className='mr-2 h-4 w-4' />
      Logout
    </Button>
  );
}
