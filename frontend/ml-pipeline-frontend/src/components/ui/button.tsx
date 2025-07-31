import * as React from 'react';
import { Slot } from '@radix-ui/react-slot';
import { cva, type VariantProps } from 'class-variance-authority';

import { cn } from '@/lib/utils';

const buttonVariants = cva(
  "inline-flex hover:cursor-pointer items-center justify-center gap-2 whitespace-nowrap rounded-sm text-sm font-medium transition-all disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 shrink-0 [&_svg]:shrink-0 outline-none focus-visible:border-accent-border focus-visible:ring-accent-ring focus-visible:ring-[3px] aria-invalid:ring-destructive/20 aria-invalid:border-destructive",
  {
    variants: {
      variant: {
        default:
          'bg-accent text-accent-foreground shadow-xs hover:bg-accent-hover',
        destructive:
          'bg-destructive text-destructive-foreground shadow-xs hover:bg-destructive-hover focus-visible:ring-destructive/40',
        outline:
          'border-1 border-accent-border bg-accent-background-1 shadow-xs text-gray-text hover:bg-accent-secondary hover:border-accent-border-hover',
        secondary:
          'bg-accent-secondary text-gray-text shadow-xs hover:bg-accent-secondary-hover',
        ghost:
          'hover:bg-accent-secondary hover:text-accent-secondary-foreground',
        link: 'text-accent underline-offset-4 hover:underline',
        icon: 'text-gray-text-muted hover:text-gray-text'
      },
      size: {
        default: 'h-9 px-4 py-2 has-[>svg]:px-3',
        sm: 'h-8 rounded-md gap-1.5 px-3 has-[>svg]:px-2.5',
        lg: 'h-10 rounded-md px-6 has-[>svg]:px-4',
        icon: 'size-9',
      },
    },
    defaultVariants: {
      variant: 'default',
      size: 'default',
    },
  }
);

export type ButtonProps = React.ComponentProps<'button'> &
  VariantProps<typeof buttonVariants> & {
    asChild?: boolean;
  };

function Button({
  className,
  variant,
  size,
  asChild = false,
  ...props
}: ButtonProps) {
  const Comp = asChild ? Slot : 'button';

  return (
    <Comp
      data-slot='button'
      type='button'
      className={cn(buttonVariants({ variant, size, className }))}
      {...props}
    />
  );
}

export { Button, buttonVariants };
