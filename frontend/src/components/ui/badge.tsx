import * as React from 'react';
import { Slot } from '@radix-ui/react-slot';
import { cva, type VariantProps } from 'class-variance-authority';

import { cn } from '@/lib/utils';

const badgeVariants = cva(
  'inline-flex items-center justify-center rounded-md border px-2 py-0.5 text-xs font-medium w-fit whitespace-nowrap shrink-0 [&>svg]:size-3 gap-1 [&>svg]:pointer-events-none focus-visible:border-accent-border focus-visible:ring-accent-ring focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive transition-[color,box-shadow] overflow-hidden',
  {
    variants: {
      variant: {
        default:
          'border-transparent bg-accent text-accent-foreground [a&]:hover:bg-accent-hover',
        secondary:
          'border-transparent bg-accent-secondary text-gray-text dark:text-gray-900 [a&]:hover:bg-accent-secondary-hover',
        destructive:
          'border-transparent bg-destructive text-destructive-foreground [a&]:hover:bg-destructive-hover focus-visible:ring-destructive/20 dark:bg-destructive/60',
        outline:
          'border-accent-border-hover text-gray-text [a&]:hover:bg-accent-secondary [a&]:hover:text-gray-text/90',
      },
      color: {
        red: 'bg-red-600 dark:bg-red-700',
        green: 'bg-green-600 dark:bg-green-700',
        blue: 'bg-blue-600 dark:bg-blue-700',
        gray: 'bg-gray-600 dark:bg-gray-700',
      },
    },
    compoundVariants: [
      {
        variant: 'secondary',
        color: 'red',
        className: 'bg-red-200 dark:bg-red-400',
      },
      {
        variant: 'secondary',
        color: 'green',
        className: 'bg-green-200 dark:bg-green-400',
      },
      {
        variant: 'secondary',
        color: 'blue',
        className: 'bg-blue-200 dark:bg-blue-400',
      },
      {
        variant: 'secondary',
        color: 'gray',
        className: 'bg-gray-200 dark:bg-gray-400',
      },
    ],
    defaultVariants: {
      variant: 'default',
    },
  }
);

function Badge({
  className,
  variant,
  asChild = false,
  color,
  ...props
}: React.ComponentProps<'span'> &
  VariantProps<typeof badgeVariants> & { asChild?: boolean }) {
  const Comp = asChild ? Slot : 'span';

  return (
    <Comp
      data-slot='badge'
      className={cn(badgeVariants({ variant, color }), className)}
      {...props}
    />
  );
}

export { Badge, badgeVariants };
