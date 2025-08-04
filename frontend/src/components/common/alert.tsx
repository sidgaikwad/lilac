import {
  Alert as AlertComponent,
  AlertDescription,
  AlertTitle,
} from '@/components/ui/alert';
import {
  CircleAlert,
  CircleCheck,
  CircleEllipsis,
  CircleHelp,
  CircleX,
  Info,
} from 'lucide-react';
import { Spinner } from './spinner/spinner';

export interface AlertProps {
  className?: string;
  variant:
    | 'default'
    | 'info'
    | 'warn'
    | 'error'
    | 'pending'
    | 'success'
    | 'help'
    | 'loading';
  title: React.ReactNode;
  description?: React.ReactNode;
  icon?: React.ReactNode;
  action?: React.ReactNode;
}

function getIcon(variant: AlertProps['variant']) {
  switch (variant) {
    case 'error':
      return <CircleX />;
    case 'warn':
      return <CircleAlert />;
    case 'info':
      return <Info />;
    case 'success':
      return <CircleCheck />;
    case 'pending':
      return <CircleEllipsis />;
    case 'loading':
      return <Spinner />;
    case 'help':
      return <CircleHelp />;
    default:
      return undefined;
  }
}

export function Alert(props: AlertProps) {
  return (
    <AlertComponent className={props.className} variant={props.variant}>
      {props.icon ?? getIcon(props.variant)}
      <AlertTitle>{props.title}</AlertTitle>
      {props.description && (
        <AlertDescription>{props.description}</AlertDescription>
      )}
      <div className='col-start-2'>{props.action}</div>
    </AlertComponent>
  );
}
