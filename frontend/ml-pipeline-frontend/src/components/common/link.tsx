import { cn } from '@/lib';
import { Link as RouterLink, LinkProps } from 'react-router-dom';

export function Link(props: LinkProps) {
  const { children, className, ...linkProps } = props;
  return (
    <RouterLink
      className={cn(
        'text-blue-600 visited:text-purple-600 hover:underline',
        className
      )}
      {...linkProps}
    >
      {children}
    </RouterLink>
  );
}
