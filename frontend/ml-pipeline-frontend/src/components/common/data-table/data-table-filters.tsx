import { Table } from '@tanstack/react-table';

interface DataTableFiltersProps<TData> {
  table: Table<TData>;
  renderFilters: (table: Table<TData>) => React.ReactNode;
}

export function DataTableFilters<TData>(props: DataTableFiltersProps<TData>) {
  return (
    <div className='flex flex-row items-center w-full max-w-sm'>
      {props.renderFilters(props.table)}
    </div>
  );
}
