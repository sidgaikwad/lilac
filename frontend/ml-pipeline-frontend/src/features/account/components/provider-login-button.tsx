import React from 'react';
import { Button } from '@/components/ui/button';
import { GoogleIcon } from '@/icons/google';
import { GithubIcon } from '@/icons/github';
import { Provider } from '@/types';

const ProviderLoginButton = ({ provider }: { provider: Provider }) => {
  const handleLogin = async () => {
    const response = await fetch(`/api/auth/${provider.type}/${provider.name}/login`, {
      method: 'POST',
    });
    const data = await response.json();

    if (data.authorization_url) {
      window.location.href = data.authorization_url;
    }
  };

  const providerName = provider.name.charAt(0).toUpperCase() + provider.name.slice(1);

  const providerIcons: { [key: string]: React.ReactNode } = {
    google: <GoogleIcon />,
    github: <GithubIcon />,
  };

  return (
    <Button
      onClick={handleLogin}
      variant="outline"
      className="w-full"
    >
      {providerIcons[provider.name]}
      <span>Continue with {providerName}</span>
    </Button>
  );
};

export default ProviderLoginButton;