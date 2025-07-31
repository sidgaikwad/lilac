import * as React from 'react';
import * as ProgressPrimitive from '@radix-ui/react-progress';

import { cn } from '@/lib/utils';
import { cva, VariantProps } from 'class-variance-authority';

const rootVariants = cva('relative h-2 w-full overflow-hidden rounded-full', {
  variants: {
    color: {
      default: 'bg-accent/20',
      red: 'bg-red-500/20',
      green: 'bg-green-500/20',
      blue: 'bg-blue-500/20',
    },
  },
  defaultVariants: {
    color: 'default',
  },
});

const indicatorVariants = cva('h-full w-full flex-1 transition-all', {
 variants: {
    color: {
      default: 'bg-accent',
      red: 'bg-red-500',
      green: 'bg-green-500',
      blue: 'bg-blue-500',
    },
  },
  defaultVariants: {
    color: 'default',
  },
});

function Progress({
  color,
  className,
  value,
  ...props
}: React.ComponentProps<typeof ProgressPrimitive.Root> &
  VariantProps<typeof rootVariants>) {
  return (
    <ProgressPrimitive.Root
      data-slot='progress'
      className={cn(
        rootVariants({
          color,
          className,
        })
      )}
      {...props}
    >
      <ProgressPrimitive.Indicator
        data-slot='progress-indicator'
        className={cn(
        indicatorVariants({
          color,
        }))}
        style={{ transform: `translateX(-${100 - (value || 0)}%)` }}
      />
    </ProgressPrimitive.Root>
  );
}

export { Progress };
