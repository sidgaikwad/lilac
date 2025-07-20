import {
  CardAction,
  Card as CardComponent,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import { cn } from '@/lib/utils';

export type CardProps = {
  className?: string;
  icon?: React.ReactNode;
  title?: React.ReactNode;
  layout?: 'horizontal' | 'vertical';
  description?: React.ReactNode;
  content?: React.ReactNode;
  footer?: React.ReactNode;
  action?: React.ReactNode;
  footerAction?: React.ReactNode;
} & Omit<React.ComponentProps<'div'>, 'content'>;

export function Card(props: CardProps) {
  const {
    className,
    icon,
    title,
    description,
    content,
    footer,
    action,
    footerAction,
    layout = 'horizontal',
    ...divProps
  } = props;
  return (
    <CardComponent {...divProps} className={cn('h-fit w-fit', className)}>
      <CardHeader>
        <div
          className={cn(
            'flex gap-4',
            layout === 'horizontal'
              ? 'flex-row items-center'
              : 'flex-col items-center text-center'
          )}
        >
          {icon}
          <div className='flex min-w-0 flex-col'>
            <CardTitle className='break-words'>{title}</CardTitle>
            <CardDescription>{description}</CardDescription>
          </div>
        </div>
        {action && <CardAction>{action}</CardAction>}
      </CardHeader>
      {content && <CardContent>{content}</CardContent>}
      {footer && (
        <CardFooter>
          {footer}
          {footerAction && <div className='ml-auto'>{footerAction}</div>}
        </CardFooter>
      )}
    </CardComponent>
  );
}
