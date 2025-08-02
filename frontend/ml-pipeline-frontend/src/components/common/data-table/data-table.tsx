import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import {
  ColumnDef,
  ColumnFiltersState,
  flexRender,
  getCoreRowModel,
  getFilteredRowModel,
  getPaginationRowModel,
  getSortedRowModel,
  PaginationState,
  RowSelectionState,
  SortingState,
  Updater,
  useReactTable,
  VisibilityState,
  Table as TanstackTable,
} from '@tanstack/react-table';
import { useCallback, useEffect, useState } from 'react';
import { DataTablePagination } from './data-table-pagination';
import { DataTableSettings } from './data-table-settings';
import { Spinner } from '../spinner/spinner';
import { DataTableFilters } from './data-table-filters';

export type Props = {
  rowSelectionProp: RowSelectionState;
  onRowSelect: (rowSelectionState: RowSelectionState) => void;
};

export const useRowSelection = ({ rowSelectionProp, onRowSelect }: Props) => {
  const [rowSelection, setRowSelection] =
    useState<RowSelectionState>(rowSelectionProp);
  useEffect(() => {
    setRowSelection(rowSelectionProp);
  }, [rowSelectionProp]);
  const handleRowSelection = useCallback(
    (nextSelectionState: Updater<RowSelectionState>) => {
      setRowSelection(nextSelectionState);
      if (typeof nextSelectionState === 'function') {
        onRowSelect(nextSelectionState(rowSelection));
      } else {
        onRowSelect(nextSelectionState);
      }
    },
    [onRowSelect, rowSelection]
  );
  return { rowSelection, handleRowSelection };
};

export interface DataTableProps<TData, TValue> {
  isLoading?: boolean;
  getRowId?: (row: TData) => string;
  selectionType?: 'single' | 'multi';
  onSelectedRowsChange?: (value: TData[]) => void;
  columns: ColumnDef<TData, TValue>[];
  renderFilters?: (table: TanstackTable<TData>) => React.ReactNode;
  data: TData[];
}

export function DataTable<TData, TValue>({
  isLoading = false,
  getRowId,
  selectionType = 'multi',
  onSelectedRowsChange,
  columns,
  renderFilters,
  data,
}: DataTableProps<TData, TValue>) {
  const [sorting, setSorting] = useState<SortingState>([]);
  const [columnFilters, setColumnFilters] = useState<ColumnFiltersState>([]);
  const [columnVisibility, setColumnVisibility] = useState<VisibilityState>({});
  const [rowSelection, setRowSelection] = useState({});
  const [pagination, setPagination] = useState<PaginationState>({
    pageIndex: 0,
    pageSize: 10,
  });
  const table = useReactTable({
    data,
    columns,
    rowCount: data.length,
    renderFallbackValue: '-',
    getCoreRowModel: getCoreRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
    onPaginationChange: setPagination,
    onSortingChange: setSorting,
    getSortedRowModel: getSortedRowModel(),
    onColumnFiltersChange: setColumnFilters,
    getFilteredRowModel: getFilteredRowModel(),
    onColumnVisibilityChange: setColumnVisibility,
    onRowSelectionChange: (updater) => {
      const selectedValues =
        updater instanceof Function ? updater(rowSelection) : updater;
      if (onSelectedRowsChange) {
        onSelectedRowsChange(
          data.filter((d, i) => selectedValues[getRowId ? getRowId(d) : i])
        );
      }
      setRowSelection(updater);
    },
    getRowId: getRowId,
    enableMultiRowSelection: selectionType === 'multi',
    state: {
      sorting,
      columnFilters,
      columnVisibility,
      rowSelection,
      pagination,
    },
  });

  return (
    <div className='space-y-1 overflow-scroll'>
      <div className='space-y-2 flex flex-row flex-wrap items-center justify-between'>
        {renderFilters && (
          <DataTableFilters table={table} renderFilters={renderFilters} />
        )}
        <div className='flex flex-row items-center'>
          <DataTablePagination table={table} />
          <DataTableSettings table={table} />
        </div>
      </div>
      <div className='overflow-hidden'>
        <Table>
          <TableHeader>
            {table.getHeaderGroups().map((headerGroup) => (
              <TableRow key={headerGroup.id}>
                {headerGroup.headers.map((header) => {
                  return (
                    <TableHead key={header.id}>
                      {header.isPlaceholder
                        ? null
                        : flexRender(
                            header.column.columnDef.header,
                            header.getContext()
                          )}
                    </TableHead>
                  );
                })}
              </TableRow>
            ))}
          </TableHeader>
          <TableBody>
            {isLoading ? (
              <TableRow>
                <TableCell
                  colSpan={columns.length}
                  className='h-24 items-center text-center'
                >
                  <span className='inline-block w-fit'>
                    <span className='text-gray-text-muted flex flex-row items-center justify-between gap-2'>
                      <Spinner />
                      Loading...
                    </span>
                  </span>
                </TableCell>
              </TableRow>
            ) : table.getRowModel().rows?.length ? (
              table.getRowModel().rows.map((row) => (
                <TableRow
                  key={row.id}
                  data-state={row.getIsSelected() && 'selected'}
                >
                  {row.getVisibleCells().map((cell) => (
                    <TableCell key={cell.id}>
                      {flexRender(
                        cell.column.columnDef.cell,
                        cell.getContext()
                      )}
                    </TableCell>
                  ))}
                </TableRow>
              ))
            ) : (
              <TableRow>
                <TableCell
                  colSpan={columns.length}
                  className='h-24 text-center'
                >
                  No results.
                </TableCell>
              </TableRow>
            )}
          </TableBody>
        </Table>
      </div>
    </div>
  );
}
