import { useEffect } from 'react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Toaster } from '@/components/ui/toast';
import { Spinner } from '@/components/ui/spinner';
import { Link, useLocation, useNavigate } from 'react-router-dom';
import { toast } from 'sonner';
import { useLogin, useGetAuthProviders } from '@/services';
import useAuthStore from '@/store/use-auth-store';
import ProviderLoginButton from '../components/provider-login-button';

const loginSchema = z.object({
  email: z.string().email({ message: 'Invalid email address' }),
  password: z.string().min(1, { message: 'Password is required' }),
});

type LoginFormInputs = z.infer<typeof loginSchema>;

function LoginPage() {
  const navigate = useNavigate();
  const { token, setToken } = useAuthStore();
  const location = useLocation();

  useEffect(() => {
    const params = new URLSearchParams(location.search);
    const error = params.get('error');
    console.log(location);
    if (error === 'duplicate_user') {
      toast.error(
        'This email is already associated with another login method.'
      );
    }
  }, [location]);

  useEffect(() => {
    if (token) {
      navigate('/organizations');
    }
  }, [token, navigate]);
  const { mutate: loginUser, isPending } = useLogin({
    onSuccess: (token) => {
      setToken(token.accessToken);
      navigate('/');
    },
    onError: (error) => {
      toast.error('Login failed', {
        description: error.error,
      });
    },
  });

  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<LoginFormInputs>({
    resolver: zodResolver(loginSchema),
  });

  // Form submission handler now directly sets auth state
  const onSubmit = (data: LoginFormInputs) => {
    loginUser({ email: data.email, password: data.password });
  };

  const { data: providers, isLoading: providersLoading } =
    useGetAuthProviders();

  return (
    <div className='bg-background flex h-screen items-center justify-center'>
      <div className='bg-card text-card-foreground w-96 rounded p-8 shadow-md'>
        <h2 className='mb-6 text-center text-2xl font-bold'>Login</h2>
        <Toaster />
        <form onSubmit={handleSubmit(onSubmit)} className='space-y-4'>
          {/* Removed general auth error display as we bypass API */}
          <div className='space-y-1'>
            <Label htmlFor='email'>Email</Label>
            <Input
              id='email'
              type='email'
              placeholder='Enter any email'
              {...register('email')}
              aria-invalid={errors.email ? 'true' : 'false'}
              disabled={isPending}
            />
            {errors.email && (
              <p className='text-destructive text-sm'>{errors.email.message}</p>
            )}
          </div>
          <div className='space-y-1'>
            <Label htmlFor='password'>Password</Label>
            <Input
              id='password'
              type='password'
              placeholder='Enter any password'
              {...register('password')}
              aria-invalid={errors.password ? 'true' : 'false'}
              disabled={isPending}
            />
            {errors.password && (
              <p className='text-destructive text-sm'>
                {errors.password.message}
              </p>
            )}
          </div>

          <div className='space-y-2'>
            <Button
              type='submit'
              className='w-full gap-4 space-y-4'
              disabled={isPending}
            >
              {isPending ? (
                <Spinner size='small' className='text-card-foreground' />
              ) : (
                <span className='text-background'>Login</span>
              )}
            </Button>
            <span className='flex justify-center-safe text-xs'>
              Don't have an account?&nbsp;
              <Link className='underline' to={{ pathname: '/signup' }}>
                Sign Up Now
              </Link>
            </span>
          </div>
        </form>
        {providers && providers.length > 0 && (
          <>
            <div className='relative my-4'>
              <div className='absolute inset-0 flex items-center'>
                <span className='w-full border-t' />
              </div>
              <div className='relative flex justify-center text-xs uppercase'>
                <span className='bg-card text-muted-foreground px-2'>or</span>
              </div>
            </div>
            <div className='space-y-2'>
              {providersLoading ? (
                <div className='flex justify-center'>
                  <Spinner />
                </div>
              ) : (
                providers?.map((provider) => (
                  <ProviderLoginButton
                    key={provider.name}
                    provider={provider}
                  />
                ))
              )}
            </div>
          </>
        )}
      </div>
    </div>
  );
}

export default LoginPage;
