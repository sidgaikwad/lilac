import { cn } from '@/lib/utils';
import { cva, VariantProps } from 'class-variance-authority';
import { CircleCheck, CircleEllipsis, CircleX, Clock, Info, Loader2, TriangleAlert } from 'lucide-react';

const statusVariants = cva('font-medium flex flex-row items-center gap-1', {
  variants: {
    status: {
      'in-progress': 'text-blue-600',
      info: 'text-blue-600',
      success: 'text-green-600',
      error: 'text-red-600',
      warning: 'text-yellow-600',
      pending: 'text-gray-600',
      loading: 'text-gray-600',
    },
    color: {
      blue: 'text-blue-600',
      green: 'text-green-600',
      red: 'text-red-600',
      yellow: 'text-yellow-600',
      gray: 'text-gray-600',
    }
  },
  defaultVariants: {
    status: 'info',
  },
});

function getIcon(status: VariantProps<typeof statusVariants>['status']) {
  switch (status) {
    case 'info':
      return <Info />;
    case 'error':
      return <CircleX />;
    case 'success':
      return <CircleCheck />;
    case 'warning':
      return <TriangleAlert />;
    case 'in-progress':
      return <CircleEllipsis />;
    case 'pending':
      return <Clock />;
    case 'loading':
      return <Loader2 className='animate-spin' />;
  }
  return undefined;
}

export type StatusProps = React.ComponentProps<'div'> &
  VariantProps<typeof statusVariants> & {
    asChild?: boolean;
  };
export function Status(props: StatusProps) {
  const { children, status, color, ...rest } = props;
  return <div className={cn(statusVariants({ status, color }))} {...rest}>{getIcon(status)}{children}</div>;
}
