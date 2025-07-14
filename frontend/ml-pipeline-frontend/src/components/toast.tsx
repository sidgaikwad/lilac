import { toast as sonnerToast } from 'sonner';
import { Button } from './ui/button';
import { CircleAlert, CircleCheck, CircleX, Info, Loader2 } from 'lucide-react';

function basic(title: string, toast: Omit<ToastProps, 'title'> = {}) {
  const { id, description, action, icon } = toast;
  return sonnerToast.custom(
    (id) => (
      <div className='bg-accent-secondary border-accent-border-hover flex w-full items-center rounded-lg border p-4 shadow-lg md:max-w-[364px]'>
        <div className='text-accent mr-3 self-start'>{icon}</div>
        <div className='flex flex-1 items-center'>
          <div className='w-full'>
            <p className='text-gray-text text-sm font-medium'>{title}</p>
            <p className='text-gray-text-muted mt-1 text-sm'>{description}</p>
          </div>
        </div>
        <div>
          {action && (
            <Button
              variant='default'
              size='sm'
              onClick={() => {
                action.onClick();
                sonnerToast.dismiss(id);
              }}
            >
              {action.label}
            </Button>
          )}
        </div>
      </div>
    ),
    id
      ? {
          id,
        }
      : undefined
  );
}

function success(title: string, toast: Omit<ToastProps, 'title'> = {}) {
  const { id, description, action, icon } = toast;
  return sonnerToast.custom(
    (id) => (
      <div className='bg-green-3 border-green-8 flex w-full items-center rounded-lg border p-4 shadow-lg md:max-w-[364px]'>
        <div className='text-green-9 mr-3 self-start'>
          {icon ?? <CircleCheck />}
        </div>
        <div className='flex flex-1 items-center'>
          <div className='w-full'>
            <p className='text-green-12 text-sm font-medium'>{title}</p>
            <p className='text-green-11 mt-1 text-sm'>{description}</p>
          </div>
        </div>
        <div>
          {action && (
            <Button
              className='bg-green-9 hover:bg-green-10 text-white'
              variant='default'
              size='sm'
              onClick={() => {
                action.onClick();
                sonnerToast.dismiss(id);
              }}
            >
              {action.label}
            </Button>
          )}
        </div>
      </div>
    ),
    id
      ? {
          id,
        }
      : undefined
  );
}

function error(title: string, toast: Omit<ToastProps, 'title'> = {}) {
  const { id, description, action, icon } = toast;
  return sonnerToast.custom(
    (id) => (
      <div className='bg-red-3 border-red-8 flex w-full items-center rounded-lg border p-4 shadow-lg md:max-w-[364px]'>
        <div className='text-red-9 mr-3 self-start'>{icon ?? <CircleX />}</div>
        <div className='flex flex-1 items-center'>
          <div className='w-full'>
            <p className='text-red-12 text-sm font-medium'>{title}</p>
            <p className='text-red-11 mt-1 text-sm'>{description}</p>
          </div>
        </div>
        <div>
          {action && (
            <Button
              className='bg-red-9 hover:bg-red-10 text-white'
              variant='default'
              size='sm'
              onClick={() => {
                action.onClick();
                sonnerToast.dismiss(id);
              }}
            >
              {action.label}
            </Button>
          )}
        </div>
      </div>
    ),
    id
      ? {
          id,
        }
      : undefined
  );
}

function info(title: string, toast: Omit<ToastProps, 'title'> = {}) {
  const { id, description, action, icon } = toast;
  return sonnerToast.custom(
    (id) => (
      <div className='bg-blue-3 border-blue-8 flex w-full items-center rounded-lg border p-4 shadow-lg md:max-w-[364px]'>
        <div className='text-blue-9 mr-3 self-start'>{icon ?? <Info />}</div>
        <div className='flex flex-1 items-center'>
          <div className='w-full'>
            <p className='text-blue-12 text-sm font-medium'>{title}</p>
            <p className='text-blue-11 mt-1 text-sm'>{description}</p>
          </div>
        </div>
        <div>
          {action && (
            <Button
              className='bg-blue-9 hover:bg-blue-10 text-white'
              variant='default'
              size='sm'
              onClick={() => {
                action.onClick();
                sonnerToast.dismiss(id);
              }}
            >
              {action.label}
            </Button>
          )}
        </div>
      </div>
    ),
    id
      ? {
          id,
        }
      : undefined
  );
}

function warning(title: string, toast: Omit<ToastProps, 'title'> = {}) {
  const { id, description, action, icon } = toast;
  return sonnerToast.custom(
    (id) => (
      <div className='bg-orange-3 border-orange-8 flex w-full items-center rounded-lg border p-4 shadow-lg md:max-w-[364px]'>
        <div className='text-orange-9 mr-3 self-start'>
          {icon ?? <CircleAlert />}
        </div>
        <div className='flex flex-1 items-center'>
          <div className='w-full'>
            <p className='text-orange-12 text-sm font-medium'>{title}</p>
            <p className='text-orange-11 mt-1 text-sm'>{description}</p>
          </div>
        </div>
        <div>
          {action && (
            <Button
              className='bg-orange-9 hover:bg-orange-10 text-white'
              variant='default'
              size='sm'
              onClick={() => {
                action.onClick();
                sonnerToast.dismiss(id);
              }}
            >
              {action.label}
            </Button>
          )}
        </div>
      </div>
    ),
    id
      ? {
          id,
        }
      : undefined
  );
}

function loading(title: string, toast: Omit<ToastProps, 'title'> = {}) {
  const { id, description, action, icon } = toast;
  return sonnerToast.custom(
    (id) => (
      <div className='bg-slate-3 border-slate-8 flex w-full items-center rounded-lg border p-4 shadow-lg md:max-w-[364px]'>
        <div className='text-slate-9 mr-3 self-start'>
          {icon ?? <Loader2 className='animate-spin' />}
        </div>
        <div className='flex flex-1 items-center'>
          <div className='w-full'>
            <p className='text-slate-12 text-sm font-medium'>{title}</p>
            <p className='text-slate-11 mt-1 text-sm'>{description}</p>
          </div>
        </div>
        <div>
          {action && (
            <Button
              className='bg-slate-9 hover:bg-slate-10 text-white'
              variant='default'
              size='sm'
              onClick={() => {
                action.onClick();
                sonnerToast.dismiss(id);
              }}
            >
              {action.label}
            </Button>
          )}
        </div>
      </div>
    ),
    id
      ? {
          id,
        }
      : undefined
  );
}

function promise<T = undefined, E = Error>(
  promise: Promise<T>,
  data?: {
    loading?: string | Omit<ToastProps, 'id'>;
    success?:
      | string
      | Omit<ToastProps, 'id'>
      | ((result: T) => Omit<ToastProps, 'id'>);
    error?:
      | string
      | Omit<ToastProps, 'id'>
      | ((error: E) => Omit<ToastProps, 'id'>);
  }
) {
  if (!data) {
    return;
  }
  let id: string | number | undefined = undefined;
  if (data.loading !== undefined) {
    const { title, ...rest } =
      typeof data.loading === 'string' ? { title: data.loading } : data.loading;
    id = loading(title, rest);
  }
  promise
    .then((res) => {
      if (data.success !== undefined) {
        if (typeof data.success === 'function') {
          const { title, ...rest } = data.success(res);
          success(title, { ...rest, id });
        } else {
          const { title, ...rest } =
            typeof data.success === 'string'
              ? { title: data.success }
              : data.success;
          success(title, { ...rest, id });
        }
      }
    })
    .catch((err) => {
      if (data.error !== undefined) {
        if (typeof data.error === 'function') {
          const { title, ...rest } = data.error(err);
          error(title, { ...rest, id });
        } else {
          const { title, ...rest } =
            typeof data.error === 'string' ? { title: data.error } : data.error;
          error(title, { ...rest, id });
        }
      }
    });
}

interface ToastProps {
  id?: string | number;
  title: string;
  description?: string;
  icon?: React.ReactNode;
  action?: {
    label: string;
    onClick: () => void;
  };
}

export const toast = Object.assign(
  basic,
  {
    success,
    info,
    warning,
    error,
    custom: sonnerToast.custom,
    message: basic,
    promise,
    dismiss: sonnerToast.dismiss,
    loading,
  },
  {
    getHistory: sonnerToast.getHistory,
    getToasts: sonnerToast.getToasts,
  }
);
