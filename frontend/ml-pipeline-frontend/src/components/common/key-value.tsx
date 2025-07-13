import { startCase } from "lodash";

export interface KeyValueDisplayProps<T> {
  data: T;
  layout?: 'vertical' | 'horizontal' | 'grid';
}

export const KeyValueDisplay = <T extends object>({
  data,
  layout = 'vertical',
}: KeyValueDisplayProps<T>) => {
  const renderItem = (key: string, value: unknown) => {
    const displayValue =
      typeof value === 'object' && value !== null
        ? JSON.stringify(value)
        : String(value);

    if (layout === 'horizontal') {
      return (
        <div
          key={key}
          className='flex items-center justify-between py-2 last:border-b-0'
        >
          <span className='font-semibold text-gray-600'>{startCase(key)}:</span>
          <span className='text-gray-800'>{displayValue}</span>
        </div>
      );
    }

    return (
      <div key={key} className='py-2'>
        <div className={'font-semibold text-gray-600'}>{startCase(key)}</div>
        <div className={'text-gray-800'}>{displayValue}</div>
      </div>
    );
  };

  const layoutClasses = {
    vertical: 'flex flex-col',
    horizontal: 'flex flex-col',
    grid: 'grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4',
  };

  return (
    <div className={layoutClasses[layout]}>
      {Object.entries(data).map(([key, value]) => renderItem(key, value))}
    </div>
  );
};
