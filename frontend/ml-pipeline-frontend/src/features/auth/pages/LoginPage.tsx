import React from 'react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { AlertCircle, Loader2Icon } from 'lucide-react'; // Added Loader2Icon
// Removed useLogin import
// import { useLogin } from '@/services/controlplane-api/user/use-login.hook';
import useAuthStore from '@/store/authStore'; // Import auth store
import { useNavigate } from 'react-router-dom'; // Import useNavigate
import { User } from '@/types'; // Import User type

const loginSchema = z.object({
  email: z.string().email({ message: "Invalid email address" }),
  password: z.string().min(1, { message: "Password is required" }),
});

type LoginFormInputs = z.infer<typeof loginSchema>;

const LoginPage: React.FC = () => {
  // Get auth store actions
  const setAuthState = useAuthStore(state => state.setAuthState);
  const navigate = useNavigate();
  const [isLoggingIn, setIsLoggingIn] = React.useState(false); // Local loading state

  // Removed useLogin hook usage

  const { register, handleSubmit, formState: { errors } } = useForm<LoginFormInputs>({
    resolver: zodResolver(loginSchema),
  });

  // Form submission handler now directly sets auth state
  const onSubmit = (data: LoginFormInputs) => {
    console.log("DEV MODE: Bypassing actual login for:", data.email);
    setIsLoggingIn(true);

    // Simulate a short delay
    setTimeout(() => {
        const mockUser: User = { id: 'dev-user', name: 'Dev User', email: data.email };
        // Set authenticated state directly in the store
        setAuthState(true, mockUser, 'mock-dev-token');
        setIsLoggingIn(false);
        // Navigate to the default protected route (e.g., pipelines or initial redirect)
        // The initial redirect logic in App.tsx will handle finding the project
        navigate('/', { replace: true });
    }, 500); // 500ms delay simulation
  };

  return (
    <div className="flex items-center justify-center h-screen bg-background">
      <div className="p-8 bg-card text-card-foreground rounded shadow-md w-96">
        <h2 className="text-2xl font-bold mb-6 text-center">Login (Dev Mode)</h2>
        <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
          {/* Removed general auth error display as we bypass API */}
          <div className="space-y-1">
            <Label htmlFor="email">Email (any valid format)</Label>
            <Input
              id="email"
              type="email"
              placeholder="Enter any email"
              {...register("email")}
              aria-invalid={errors.email ? "true" : "false"}
              disabled={isLoggingIn}
            />
            {errors.email && <p className="text-sm text-destructive">{errors.email.message}</p>}
          </div>
          <div className="space-y-1">
            <Label htmlFor="password">Password (any)</Label>
            <Input
              id="password"
              type="password"
              placeholder="Enter any password"
              {...register("password")}
              aria-invalid={errors.password ? "true" : "false"}
              disabled={isLoggingIn}
            />
            {errors.password && <p className="text-sm text-destructive">{errors.password.message}</p>}
          </div>

          <Button type="submit" className="w-full" disabled={isLoggingIn}>
            {isLoggingIn ? <Loader2Icon className="mr-2 h-4 w-4 animate-spin" /> : null}
            {isLoggingIn ? 'Logging in...' : 'Login (Bypass)'}
          </Button>
        </form>
      </div>
    </div>
  );
};

export default LoginPage;