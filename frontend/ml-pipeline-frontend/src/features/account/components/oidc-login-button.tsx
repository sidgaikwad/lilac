import { Button } from '@/components/ui/button';

const OidcLoginButton = ({ provider }: { provider: string }) => {
  const handleLogin = () => {
    window.location.href = `/api/auth/oidc/login/${provider}`;
  };

  return (
    <Button onClick={handleLogin} className="w-full">
      Login with {provider}
    </Button>
  );
};

export default OidcLoginButton;