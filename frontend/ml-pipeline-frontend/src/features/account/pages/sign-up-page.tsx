import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Toaster } from '@/components/ui/sonner';
import { Spinner } from '@/components/ui/spinner';
import { toast } from '@/components/toast';
import { Link, useNavigate } from 'react-router-dom';
import { useSignUp } from '@/services';
import { Card } from '@/components/common/card';

const usernameRegex = /^[a-zA-Z0-9_-]+$/;
const registerSchema = z
  .object({
    firstName: z.string().optional(),
    lastName: z.string().optional(),
    username: z.string().min(3).refine((username) => usernameRegex.test(username), 'Must only contain -, _, or alphanumeric characters.'),
    password: z
      .string()
      .min(8, { message: 'Password must be at least 8 characters' }),
    password2: z.string(),
  })
  .refine((values) => values.password === values.password2, {
    message: 'Passwords do not match',
    path: ['password2'],
  });

type RegisterFormInputs = z.infer<typeof registerSchema>;

function SignUpPage() {
  const navigate = useNavigate();
  // Get auth store actions
  const { mutate: signUp, isPending } = useSignUp({
    onSuccess: () => {
      toast.success('Successfully signed up!');
      setTimeout(() => navigate('/login'), 1000);
    },
    onError: (error) => {
      toast.error(error.error);
    },
  });

  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<RegisterFormInputs>({
    resolver: zodResolver(registerSchema),
  });

  // Form submission handler now directly sets auth state
  const onSubmit = (data: RegisterFormInputs) => {
    signUp({ firstName: data.firstName, lastName: data.lastName, username: data.username, password: data.password });
  };

  return (
    <div className='flex h-screen w-full items-center justify-center'>
      <Toaster />
      <Card
        className='min-w-sm'
        title='Sign Up'
        content={
          <div>
            <form onSubmit={handleSubmit(onSubmit)} className='space-y-4'>
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
                  placeholder='Enter password'
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
              <div className='space-y-1'>
                <Label htmlFor='password'>Confirm Password</Label>
                <Input
                  id='confirm-password'
                  type='password'
                  placeholder='Confirm password'
                  {...register('password2')}
                  aria-invalid={errors.password2 ? 'true' : 'false'}
                  disabled={isPending}
                />
                {errors.password2 && (
                  <p className='text-destructive text-sm'>
                    {errors.password2.message}
                  </p>
                )}
              </div>

              <div className='space-y-2'>
                <Button type='submit' className='w-full' disabled={isPending}>
                  {isPending ? (
                    <Spinner size='small' />
                  ) : (
                    <span className='text-background'>Sign Up</span>
                  )}
                </Button>
                <span className='flex justify-center-safe text-xs'>
                  Already have an account?&nbsp;
                  <Link className='underline' to={{ pathname: '/login' }}>
                    Sign In Now
                  </Link>
                </span>
              </div>
            </form>
          </div>
        }
      />
    </div>
  );
}

export default SignUpPage;
