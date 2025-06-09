import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Toaster } from '@/components/ui/toast';
import { Spinner } from '@/components/ui/spinner';
import { toast } from 'sonner';
import { Link, useNavigate } from 'react-router-dom';
import { useSignUp } from '@/services';

const registerSchema = z
  .object({
    email: z.string().email({ message: 'Invalid email address' }),
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
    signUp({ email: data.email, password: data.password });
  };

  return (
    <div className='bg-background flex h-screen items-center justify-center'>
      <div className='bg-card text-card-foreground w-96 rounded p-8 shadow-md'>
        <h2 className='mb-6 text-center text-2xl font-bold'>Sign Up</h2>
        <Toaster />
        <form onSubmit={handleSubmit(onSubmit)} className='space-y-4'>
          {/* Removed general auth error display as we bypass API */}
          <div className='space-y-1'>
            <Label htmlFor='email'>Email</Label>
            <Input
              id='email'
              type='email'
              placeholder='Enter your email'
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
    </div>
  );
}

export default SignUpPage;
