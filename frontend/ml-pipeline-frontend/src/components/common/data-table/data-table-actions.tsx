import { Table } from '@tanstack/react-table';

import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { Button } from '@/components/ui/button';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog';
import { Settings } from 'lucide-react';
import { startCase } from 'lodash';
import { Switch } from '@/components/ui/switch';

interface DataTableSettingsProps<TData> {
  table: Table<TData>;
}

export function DataTableSettings<TData>({
  table,
}: DataTableSettingsProps<TData>) {
  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button variant='icon'>
          <Settings />
        </Button>
      </DialogTrigger>
      <DialogContent className='h-fit'>
        <DialogHeader>
          <DialogTitle>Table Settings</DialogTitle>
          <DialogDescription></DialogDescription>
        </DialogHeader>
        <div className='grid grid-cols-2 p-2'>
          <div className='justify-left flex flex-col space-x-2'>
            <p className='text-sm font-medium'>Rows per page</p>
            <Select
              value={`${table.getState().pagination.pageSize}`}
              onValueChange={(value) => {
                table.setPageSize(Number(value));
              }}
            >
              <SelectTrigger className='h-8 w-[70px]'>
                <SelectValue
                  placeholder={table.getState().pagination.pageSize}
                />
              </SelectTrigger>
              <SelectContent side='top'>
                {[10, 20, 25, 30, 40, 50].map((pageSize) => (
                  <SelectItem key={pageSize} value={`${pageSize}`}>
                    {pageSize}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
          <div className='justify-left flex flex-col space-x-2'>
            <p className='text-sm font-medium'>Columns</p>
            {table
              .getAllColumns()
              .filter((column) => column.getCanHide())
              .map((column) => {
                return (
                  <div className='flex flex-row items-center gap-2'>
                    <Switch
                      key={column.id}
                      checked={column.getIsVisible()}
                      onCheckedChange={(value) =>
                        column.toggleVisibility(!!value)
                      }
                    />
                    {startCase(column.id)}
                  </div>
                );
              })}
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
