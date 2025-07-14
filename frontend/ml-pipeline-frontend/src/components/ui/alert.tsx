import * as React from 'react';
import { cva, type VariantProps } from 'class-variance-authority';

import { cn } from '@/lib/utils';

const alertVariants = cva(
  'relative w-full shadow-sm rounded-lg border border-accent-border-subtle ring-accent-ring px-4 py-3 text-sm grid has-[>svg]:grid-cols-[calc(var(--spacing)*4)_1fr] grid-cols-[0_1fr] has-[>svg]:gap-x-3 gap-y-0.5 items-start [&>svg]:size-5 [&>svg]:translate-y-0.5 [&>svg]:text-current',
  {
    variants: {
      variant: {
        default: 'bg-accent-background-1 text-gray-text',
        info: 'bg-blue-2 text-blue-12 border-blue-8 [&>svg]:text-blue-11 *:data-[slot=alert-description]:text-blue-11',
        pending:
          'bg-slate-2 border-slate-8 text-slate-12 [&>svg]:text-slate-11 *:data-[slot=alert-description]:text-slate-11',
        loading:
          'bg-slate-2 border-slate-8 text-slate-12 [&>svg]:text-slate-11 *:data-[slot=alert-description]:text-slate-11',
        warn: 'bg-orange-2 text-orange-12 border-orange-8 [&>svg]:text-orange-11 *:data-[slot=alert-description]:text-orange-11',
        alert:
          'bg-orange-2 text-orange-12 border-orange-8 [&>svg]:text-orange-11 *:data-[slot=alert-description]:text-orange-11',
        success:
          'bg-green-2 border-green-8 text-green-12  [&>svg]:text-green-11 *:data-[slot=alert-description]:text-green-11',
        help: 'bg-iris-2 border-iris-8 text-iris-12 [&>svg]:text-iris-11 *:data-[slot=alert-description]:text-iris-11',
        error:
          'bg-red-2 border-red-8 text-red-12 [&>svg]:text-red-11 *:data-[slot=alert-description]:text-red-11',
        destructive:
          'bg-red-2 border-red-8 text-red-12 [&>svg]:text-red-11 *:data-[slot=alert-description]:text-red-11',
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
        'text-gray-foreground-muted col-start-2 grid justify-items-start gap-1 text-sm [&_p]:leading-relaxed',
        className
      )}
      {...props}
    />
  );
}

export { Alert, AlertTitle, AlertDescription };
