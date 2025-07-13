import {
  CardAction,
  Card as CardComponent,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';

export type CardProps = {
  className?: string;
  icon?: React.ReactNode;
  title?: React.ReactNode;
  description?: React.ReactNode;
  content?: React.ReactNode;
  footer?: React.ReactNode;
  action?: React.ReactNode;
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
    ...divProps
  } = props;
  return (
    <CardComponent {...divProps} className={className}>
      <CardHeader>
        <div className='flex flex-row items-center gap-4'>
          {icon}
          <div className='flex flex-col'>
            <CardTitle>{title}</CardTitle>
            <CardDescription>{description}</CardDescription>
          </div>
        </div>
        {action && <CardAction>{action}</CardAction>}
      </CardHeader>
      {content && <CardContent>{content}</CardContent>}
      {footer && <CardFooter>{footer}</CardFooter>}
    </CardComponent>
  );
}
