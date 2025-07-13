import * as React from 'react';
import { cva, type VariantProps } from 'class-variance-authority';

import { cn } from '@/lib/utils';

const alertVariants = cva(
  'relative w-full rounded-lg border px-4 py-3 text-sm grid has-[>svg]:grid-cols-[calc(var(--spacing)*4)_1fr] grid-cols-[0_1fr] has-[>svg]:gap-x-3 gap-y-0.5 items-start [&>svg]:size-4 [&>svg]:translate-y-0.5 [&>svg]:text-current',
  {
    variants: {
      variant: {
        default: 'bg-card text-card-foreground',
        info: 'bg-blue-300/10 text-blue-500 border-blue-500 [&>svg]:text-current *:data-[slot=alert-description]:text-blue-500/90',
        pending:
          'bg-gray-300/10 border-gray-700 text-gray-700 [&>svg]:text-current *:data-[slot=alert-description]:text-gray-700/90',
        loading:
          'bg-gray-300/10 border-gray-700 text-gray-700 [&>svg]:text-current *:data-[slot=alert-description]:text-gray-700/90',
        warn: 'bg-orange-300/10 text-orange-500 border-orange-500 [&>svg]:text-current *:data-[slot=alert-description]:text-orange-500/90',
        alert:
          'bg-card text-orange-500 [&>svg]:text-current *:data-[slot=alert-description]:text-orange-500/90',
        success:
          'bg-green-300/10 text-green-600 border-green-600 [&>svg]:text-current *:data-[slot=alert-description]:text-green-600/90',
        help: 'bg-slate-300/10 border-slate-700 text-slate-700 [&>svg]:text-current *:data-[slot=alert-description]:text-slate-700/90',
        error:
          'text-destructive bg-red-300/10 border-red-500 [&>svg]:text-current *:data-[slot=alert-description]:text-destructive/90',
        destructive:
          'text-destructive bg-card [&>svg]:text-current *:data-[slot=alert-description]:text-destructive/90',
      },
    },
    defaultVariants: {
      variant: 'default',
    },
  }
);

function Alert({
  className,
  variant,
  ...props
}: React.ComponentProps<'div'> & VariantProps<typeof alertVariants>) {
  return (
    <div
      data-slot='alert'
      role='alert'
      className={cn(alertVariants({ variant }), className)}
      {...props}
    />
  );
}

function AlertTitle({ className, ...props }: React.ComponentProps<'div'>) {
  return (
    <div
      data-slot='alert-title'
      className={cn(
        'col-start-2 line-clamp-1 min-h-4 font-medium tracking-tight',
        className
      )}
      {...props}
    />
  );
}

function AlertDescription({
  className,
  ...props
}: React.ComponentProps<'div'>) {
  return (
    <div
      data-slot='alert-description'
      className={cn(
        'text-muted-foreground col-start-2 grid justify-items-start gap-1 text-sm [&_p]:leading-relaxed',
        className
      )}
      {...props}
    />
  );
}

export { Alert, AlertTitle, AlertDescription };
