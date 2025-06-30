import { Button } from '@/components/ui/button';
import { GoogleIcon } from '@/icons/google';

const OidcLoginButton = ({ provider }: { provider: string }) => {
  const handleLogin = () => {
    window.location.href = `/api/auth/oidc/login/${provider}`;
  };

  const providerName = provider.charAt(0).toUpperCase() + provider.slice(1);

  return (
    <Button
      onClick={handleLogin}
      variant="outline"
      className="w-full"
    >
      <GoogleIcon />
      <span>Continue with {providerName}</span>
    </Button>
  );
};

export default OidcLoginButton;