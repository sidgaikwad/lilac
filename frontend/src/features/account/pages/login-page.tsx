import { useEffect } from 'react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Toaster } from '@/components/ui/sonner';
import { Spinner } from '@/components/ui/spinner';
import { Link, useNavigate } from 'react-router-dom';
import { useLogin } from '@/services';
import useAuthStore from '@/store/use-auth-store';
import { toast } from '@/components/toast';
import { Card } from '@/components/common/card';

const usernameRegex = /^[a-zA-Z0-9_-]+$/;
const loginSchema = z.object({
  username: z
    .string()
    .min(3)
    .refine(
      (username) => usernameRegex.test(username),
      'Must only contain -, _, or alphanumeric characters.'
    ),
  password: z.string().min(1, { message: 'Password is required' }),
});

type LoginFormInputs = z.infer<typeof loginSchema>;

function LoginPage() {
  const navigate = useNavigate();
  const { token, setToken } = useAuthStore();

  useEffect(() => {
    if (token !== null) {
      navigate('/');
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
    loginUser({ username: data.username, password: data.password });
  };

  return (
    <div className='bg-background flex h-screen items-center justify-center'>
      <Toaster />
      <Card
        className='min-w-sm'
        title='Login'
        content={
          <form onSubmit={handleSubmit(onSubmit)} className='space-y-4'>
            {/* Removed general auth error display as we bypass API */}
            <div className='space-y-1'>
              <Label htmlFor='username'>Username</Label>
              <Input
                id='username'
                type='text'
                placeholder='Enter your username'
                {...register('username')}
                aria-invalid={errors.username ? 'true' : 'false'}
                disabled={isPending}
              />
              {errors.username && (
                <p className='text-destructive text-sm'>
                  {errors.username.message}
                </p>
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
        }
      />
    </div>
  );
}

export default LoginPage;
