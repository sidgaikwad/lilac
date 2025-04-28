import React from 'react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { useLogin } from '@/services/controlplane-api/useLogin.hook';
import { Spinner, Toaster } from '@/components/ui';
import { Link } from 'react-router-dom';

const loginSchema = z.object({
  email: z.string().email({ message: 'Invalid email address' }),
  password: z.string().min(1, { message: 'Password is required' }),
});

type LoginFormInputs = z.infer<typeof loginSchema>;

const LoginPage: React.FC = () => {
  // Get auth store actions
  const { mutate: loginUser, isPending } = useLogin({ redirectTo: '/' });

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

  return (
    <div className="flex items-center justify-center h-screen bg-background">
      <div className="p-8 bg-card text-card-foreground rounded shadow-md w-96">
        <h2 className="text-2xl font-bold mb-6 text-center">Login</h2>
        <Toaster />
        <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
          {/* Removed general auth error display as we bypass API */}
          <div className="space-y-1">
            <Label htmlFor="email">Email</Label>
            <Input
              id="email"
              type="email"
              placeholder="Enter any email"
              {...register('email')}
              aria-invalid={errors.email ? 'true' : 'false'}
              disabled={isPending}
            />
            {errors.email && (
              <p className="text-sm text-destructive">{errors.email.message}</p>
            )}
          </div>
          <div className="space-y-1">
            <Label htmlFor="password">Password</Label>
            <Input
              id="password"
              type="password"
              placeholder="Enter any password"
              {...register('password')}
              aria-invalid={errors.password ? 'true' : 'false'}
              disabled={isPending}
            />
            {errors.password && (
              <p className="text-sm text-destructive">
                {errors.password.message}
              </p>
            )}
          </div>

          <div className="space-y-2">
            <Button
              type="submit"
              className="w-full gap-4 space-y-4"
              disabled={isPending}
            >
              {isPending ? (
                <Spinner size="small" className="text-card-foreground" />
              ) : (
                <span className="text-background">Login</span>
              )}
            </Button>
            <span className="flex justify-center-safe text-xs">
              Don't have an account?&nbsp;
              <Link className="underline" to={{ pathname: '/signup' }}>
                Sign Up Now
              </Link>
            </span>
          </div>
        </form>
      </div>
    </div>
  );
};

export default LoginPage;
