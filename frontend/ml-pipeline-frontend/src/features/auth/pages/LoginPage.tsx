import React from 'react';

const LoginPage: React.FC = () => {
  return (
    <div className="flex items-center justify-center h-screen bg-gray-100 dark:bg-gray-900">
      <div className="p-8 bg-white dark:bg-gray-800 rounded shadow-md w-96">
        <h2 className="text-2xl font-bold mb-6 text-center text-gray-900 dark:text-gray-100">Login</h2>
        {/* TODO: Implement Login Form using react-hook-form and shadcn/ui components */}
        <p className="text-center text-gray-600 dark:text-gray-400">Login form placeholder.</p>
        {/* TODO: Add API call using apiClient and authStore */}
      </div>
    </div>
  );
};

export default LoginPage;