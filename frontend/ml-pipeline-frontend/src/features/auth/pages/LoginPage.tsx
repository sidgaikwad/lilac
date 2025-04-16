import React from 'react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import useAuthStore from '@/store/authStore';
import { useNavigate } from 'react-router-dom';
import { toast } from 'sonner';
import { AlertCircle } from 'lucide-react';
import { useMutation } from '@tanstack/react-query'; // Import useMutation
import { loginUser } from '../services/authService'; // Import the API call function

const loginSchema = z.object({
  email: z.string().email({ message: "Invalid email address" }),
  password: z.string().min(1, { message: "Password is required" }),
});

type LoginFormInputs = z.infer<typeof loginSchema>;

const LoginPage: React.FC = () => {
  const navigate = useNavigate();
  // Get state setters from Zustand store
  const setAuthState = useAuthStore((state) => state.setAuthState);
  const setError = useAuthStore((state) => state.setError);
  const authError = useAuthStore((state) => state.error); // Still need error for display

  const { register, handleSubmit, formState: { errors } } = useForm<LoginFormInputs>({
    resolver: zodResolver(loginSchema),
  });

  // Setup mutation using React Query
  const loginMutation = useMutation({
    mutationFn: loginUser, // Function that performs the API call
    onSuccess: (data) => {
      // On successful API call, update Zustand store
      setAuthState(true, data.user, data.token);
      setError(null); // Clear any previous errors
      navigate('/pipelines'); // Redirect on success
    },
    onError: (error) => {
      // On API error, update Zustand error state
      const errorMsg = error instanceof Error ? error.message : "Login failed";
      setError(errorMsg);
      toast.error("Login Failed", { description: errorMsg });
    },
  });

  // Form submission handler now calls the mutation
  const onSubmit = (data: LoginFormInputs) => {
    setError(null); // Clear previous errors before submitting
    loginMutation.mutate(data); // Execute the mutation
  };

  return (
    <div className="flex items-center justify-center h-screen bg-background">
      <div className="p-8 bg-card text-card-foreground rounded shadow-md w-96">
        <h2 className="text-2xl font-bold mb-6 text-center">Login</h2>
        <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
          {/* Display general auth error from store */}
          {authError && (
             <div className="p-3 rounded-md bg-destructive/10 text-destructive text-sm flex items-center gap-2">
                <AlertCircle className="h-4 w-4"/>
                <span>{authError}</span>
             </div>
          )}
          <div className="space-y-1">
            <Label htmlFor="email">Email (admin@example.com)</Label>
            <Input
              id="email"
              type="email"
              placeholder="Enter your email"
              {...register("email")}
              aria-invalid={errors.email ? "true" : "false"}
              disabled={loginMutation.isPending} // Disable while logging in
            />
            {errors.email && <p className="text-sm text-destructive">{errors.email.message}</p>}
          </div>
          <div className="space-y-1">
            <Label htmlFor="password">Password (admin)</Label>
            <Input
              id="password"
              type="password"
              placeholder="Enter your password"
              {...register("password")}
              aria-invalid={errors.password ? "true" : "false"}
              disabled={loginMutation.isPending} // Disable while logging in
            />
            {errors.password && <p className="text-sm text-destructive">{errors.password.message}</p>}
          </div>

          <Button type="submit" className="w-full" disabled={loginMutation.isPending}>
            {loginMutation.isPending ? 'Logging in...' : 'Login'}
          </Button>
        </form>
      </div>
    </div>
  );
};

export default LoginPage;