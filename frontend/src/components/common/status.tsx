import { cn } from '@/lib/utils';
import { cva, VariantProps } from 'class-variance-authority';
import {
  CircleCheck,
  CircleEllipsis,
  CircleSlash,
  CircleX,
  Clock,
  Info,
  TriangleAlert,
} from 'lucide-react';
import { Spinner } from './spinner/spinner';

const statusVariants = cva(
  'w-fit px-3 py-1 rounded-md font-medium flex grow-0 flex-row items-center gap-2',
  {
    variants: {
      status: {
        'in-progress': 'text-blue-700 bg-blue-100 dark:bg-blue-900 dark:text-blue-100',
        info: 'text-blue-700 bg-blue-100 dark:bg-blue-900 dark:text-blue-100',
        success: 'text-green-700 bg-green-100 dark:bg-green-900 dark:text-green-100',
        error: 'text-red-700 bg-red-100 dark:bg-red-900 dark:text-red-100',
        warning: 'text-yellow-700 bg-yellow-100 dark:bg-yellow-900 dark:text-yellow-100',
        pending: 'text-gray-700 bg-gray-100 dark:bg-gray-800 dark:text-gray-100',
        loading: 'text-gray-700 bg-gray-100 dark:bg-gray-800 dark:text-gray-100',
        cancelled: 'text-yellow-700 bg-yellow-100 dark:bg-yellow-900 dark:text-yellow-100',
      },
      color: {
        blue: 'text-blue-700 bg-blue-100 dark:bg-blue-900 dark:text-blue-100',
        green: 'text-green-700 bg-green-100 dark:bg-green-900 dark:text-green-100',
        red: 'text-red-700 bg-red-100 dark:bg-red-900 dark:text-red-100',
        yellow: 'text-yellow-700 bg-yellow-100 dark:bg-yellow-900 dark:text-yellow-100',
        gray: 'text-gray-700 bg-gray-100 dark:bg-gray-800 dark:text-gray-100',
      },
    },
  }
);

function getIcon(status: VariantProps<typeof statusVariants>['status']) {
  switch (status) {
    case 'info':
      return <Info className='size-4.5' />;
    case 'error':
      return <CircleX className='size-4.5' />;
    case 'success':
      return <CircleCheck className='size-4.5' />;
    case 'warning':
      return <TriangleAlert className='size-4.5' />;
    case 'in-progress':
      return <CircleEllipsis className='size-4.5' />;
    case 'pending':
      return <Clock className='size-4.5' />;
    case 'cancelled':
      return <CircleSlash className='size-4.5' />;
    case 'loading':
      return <Spinner className='size-4.5' />;
  }
  return undefined;
}

export type StatusProps = React.ComponentProps<'div'> &
  VariantProps<typeof statusVariants> & {
    asChild?: boolean;
    icon?: React.ReactNode;
  };
export function Status(props: StatusProps) {
  const { children, status, color, className, icon, ...rest } = props;
  return (
    <div className={cn(statusVariants({ status, color }), className)} {...rest}>
      {icon ?? getIcon(status)}
      {children}
    </div>
  );
}
