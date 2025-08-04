import { Column } from '@tanstack/react-table';
import { ChevronDown, ChevronsUpDown, ChevronUp } from 'lucide-react';

import { cn } from '@/lib/utils';
import { Button } from '@/components/ui/button';

interface DataTableColumnHeaderProps<TData, TValue>
  extends React.HTMLAttributes<HTMLDivElement> {
  column: Column<TData, TValue>;
  title: string;
  sortable?: boolean;
}

export function DataTableColumnHeader<TData, TValue>({
  column,
  title,
  className,
}: DataTableColumnHeaderProps<TData, TValue>) {
  if (!column.getCanSort()) {
    return <div className={cn(className)}>{title}</div>;
  }

  return (
    <div className={cn('flex items-center gap-2', className)}>
      <Button
        variant='ghost'
        size='sm'
        className='data-[state=open]:bg-accent-secondary-focus -ml-3 h-8'
        onClick={() => column.toggleSorting()}
      >
        <span>{title}</span>
        {column.getIsSorted() === 'desc' ? (
          <ChevronDown />
        ) : column.getIsSorted() === 'asc' ? (
          <ChevronUp />
        ) : (
          <ChevronsUpDown />
        )}
      </Button>
    </div>
  );
}
