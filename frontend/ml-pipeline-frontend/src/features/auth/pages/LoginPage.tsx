import React from 'react';

const LoginPage: React.FC = () => {
  return (
    // Use theme background
    <div className="flex items-center justify-center h-screen bg-background">
      {/* Use theme card background and text color */}
      <div className="p-8 bg-card text-card-foreground rounded shadow-md w-96">
        <h2 className="text-2xl font-bold mb-6 text-center">Login</h2>
        {/* TODO: Implement Login Form using react-hook-form and shadcn/ui components */}
        {/* Use theme muted text color */}
        <p className="text-center text-muted-foreground">Login form placeholder.</p>
        {/* TODO: Add API call using apiClient and authStore */}
      </div>
    </div>
  );
};

export default LoginPage;