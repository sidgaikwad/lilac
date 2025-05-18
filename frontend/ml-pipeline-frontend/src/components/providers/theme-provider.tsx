import React, { createContext, useContext, useState, useEffect } from 'react';

type Theme = 'dark' | 'light' | 'system';

interface ThemeProviderState {
  theme: Theme;
  setTheme: (theme: Theme) => void;
}

const initialState: ThemeProviderState = {
  theme: 'system',
  setTheme: () => null,
};

const ThemeProviderContext = createContext<ThemeProviderState>(initialState);

interface ThemeProviderProps {
  children: React.ReactNode;
  defaultTheme?: Theme;
  storageKey?: string;
}

export function ThemeProvider({
  children,
  defaultTheme = 'system',
  storageKey = 'lilac-ui-theme', // Use a specific key for this app
  ...props
}: ThemeProviderProps) {
  const [theme, setTheme] = useState<Theme>(
    () => (localStorage.getItem(storageKey) as Theme) || defaultTheme
  );

  useEffect(() => {
    const root = window.document.documentElement;

    root.classList.remove('light', 'dark');

    let systemTheme: Theme = 'light'; // Default system theme
    if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
      systemTheme = 'dark';
    }

    const currentTheme = theme === 'system' ? systemTheme : theme;

    root.classList.add(currentTheme);
  }, [theme]);

  const value = {
    theme,
    setTheme: (newTheme: Theme) => {
      localStorage.setItem(storageKey, newTheme);
      setTheme(newTheme);
    },
  };

  return (
    <ThemeProviderContext.Provider {...props} value={value}>
      {children}
    </ThemeProviderContext.Provider>
  );
}

// Custom hook to use the theme context
export const useTheme = () => {
  const context = useContext(ThemeProviderContext);

  if (context === undefined)
    throw new Error('useTheme must be used within a ThemeProvider');

  return context;
};
