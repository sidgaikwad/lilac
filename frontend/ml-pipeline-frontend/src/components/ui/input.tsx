import * as React from 'react';

import { cn } from '@/lib/utils';

function Input({ className, type, ...props }: React.ComponentProps<'input'>) {
  return (
    <input
      type={type}
      data-slot="input"
      className={cn(
        // Base styles
        'flex h-9 w-full min-w-0 rounded-md border bg-transparent px-3 py-1 text-base shadow-xs transition-[color,box-shadow] outline-none',
        // File input specific styles
        'file:text-foreground file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium',
        // Placeholder styles
        'placeholder:text-muted-foreground',
        // Selection styles
        'selection:bg-primary selection:text-primary-foreground',
        // Disabled styles
        'disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50',
        // Focus visible styles (using theme ring)
        'focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]',
        // Invalid styles (using theme destructive)
        'aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive',
        // Theme-specific styles
        'border-input', // Use theme border color
        'dark:bg-input/30', // Dark mode background (adjust opacity if needed)
        'dark:text-foreground', // Ensure text is visible in dark mode
        // Responsive text size
        'md:text-sm',
        className
      )}
      {...props}
    />
  );
}

export { Input };
