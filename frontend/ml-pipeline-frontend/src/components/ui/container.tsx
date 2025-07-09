import * as React from 'react';

import { cn } from '@/lib/utils';

function Container({ className, ...props }: React.ComponentProps<'div'>) {
  return (
    <div
      data-slot='container'
      className={cn(
        'bg-container text-container-foreground @container flex h-full w-full flex-col gap-6 py-6 shadow-sm md:px-12',
        className
      )}
      {...props}
    />
  );
}

function ContainerHeader({ className, ...props }: React.ComponentProps<'div'>) {
  return (
    <div
      data-slot='container-header'
      className={cn(
        '@container/container-header flex flex-row flex-wrap items-center justify-between gap-x-8 gap-y-2 px-6 [.border-b]:pb-6',
        className
      )}
      {...props}
    />
  );
}

function ContainerTitle({ className, ...props }: React.ComponentProps<'div'>) {
  return (
    <div
      data-slot='container-title'
      className={cn('peer text-2xl leading-none font-semibold', className)}
      {...props}
    />
  );
}

function ContainerDescription({
  className,
  ...props
}: React.ComponentProps<'div'>) {
  return (
    <div
      data-slot='container-description'
      className={cn(
        'text-muted-foreground pt-1 align-bottom text-sm font-normal peer-[[data-slot=container-title]]:basis-full',
        className
      )}
      {...props}
    />
  );
}

function ContainerAction({ className, ...props }: React.ComponentProps<'div'>) {
  return (
    <div
      data-slot='container-action'
      className={cn(
        'md:elf-start space-y-1 space-x-2 justify-self-end pt-2 md:pt-0',
        className
      )}
      {...props}
    />
  );
}

function ContainerContent({
  className,
  ...props
}: React.ComponentProps<'div'>) {
  return (
    <div
      data-slot='container-content'
      className={cn('flex-1 h-full w-full space-y-8 px-6', className)}
      {...props}
    />
  );
}

function ContainerFooter({ className, ...props }: React.ComponentProps<'div'>) {
  return (
    <div
      data-slot='container-footer'
      className={cn('flex items-center px-6 [.border-t]:pt-6', className)}
      {...props}
    />
  );
}

export {
  Container,
  ContainerHeader,
  ContainerFooter,
  ContainerTitle,
  ContainerAction,
  ContainerDescription,
  ContainerContent,
};
