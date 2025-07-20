import React from 'react';
// import { Button } from '@/components/ui/button';
import { GoogleIcon } from '@/icons/google';
import { GithubIcon } from '@/icons/github';
import { Provider } from '@/types';

// import { useSsoLogin } from '@/services';

const ProviderLoginButton = ({ provider }: { provider: Provider }) => {
  // const { mutate: ssoLogin } = useSsoLogin();

  // const handleLogin = () => {
  //   ssoLogin({ provider: provider.name, type: provider.type });
  // };

  const providerName =
    provider.name.charAt(0).toUpperCase() + provider.name.slice(1);

  const providerIcons: { [key: string]: React.ReactNode } = {
    google: <GoogleIcon />,
    github: <GithubIcon />,
  };

  return (
    <></>
    // <Button onClick={handleLogin} variant='outline' className='w-full'>
    //   {providerIcons[provider.name]}
    //   <span>Continue with {providerName}</span>
    // </Button>
  );
};

export default ProviderLoginButton;
