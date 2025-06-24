import { Button } from '@/components/ui/button';
import { Toaster } from '@/components/ui/toast';
import { Spinner } from '@/components/ui/spinner';
import { useGetOidcProviders } from '@/services/auth/get-oidc-providers.query';

function OidcLoginPage() {
  const { data: providers, isLoading } = useGetOidcProviders();

  const handleLogin = (provider: string) => {
    window.location.href = `/api/auth/oidc/login/${provider}`;
  };

  return (
    <div className='bg-background flex h-screen items-center justify-center'>
      <div className='bg-card text-card-foreground w-96 rounded p-8 shadow-md'>
        <h2 className='mb-6 text-center text-2xl font-bold'>Login with OIDC</h2>
        <Toaster />
        <div className='space-y-4'>
          {isLoading ? (
            <div className='flex justify-center'>
              <Spinner />
            </div>
          ) : (
            providers?.map((provider) => (
              <Button
                key={provider}
                onClick={() => handleLogin(provider)}
                className='w-full'
              >
                Sign in with {provider}
              </Button>
            ))
          )}
        </div>
      </div>
    </div>
  );
}

export default OidcLoginPage;