import { forwardRef } from 'react';
import { Handle, HandleProps } from '@xyflow/react';

import { cn } from '@/lib/utils';

export type BaseHandleProps = HandleProps;

export const BaseHandle = forwardRef<HTMLDivElement, BaseHandleProps>(
  ({ className, children, ...props }, ref) => {
    return (
      <Handle
        ref={ref}
        {...props}
        className={cn(
          'dark:border-secondary dark:bg-secondary size-[11px] h-[11px] w-[11px] rounded-full border border-slate-300 bg-slate-100 p-1 transition',
          className
        )}
        {...props}
      >
        {children}
      </Handle>
    );
  }
);

BaseHandle.displayName = 'BaseHandle';
