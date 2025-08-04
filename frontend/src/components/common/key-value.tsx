import { cva } from 'class-variance-authority';

const keyValueVariants = cva('gap-y-2', {
  variants: {
    layout: {
      vertical: 'flex flex-col',
      horizontal: 'flex flex-col',
      grid: 'grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4',
    },
  },
  defaultVariants: {
    layout: 'vertical',
  },
});

export interface KeyValueItem {
  key: React.ReactNode;
  value: React.ReactNode;
}

export interface KeyValueDisplayProps {
  items: KeyValueItem[];
  layout?: 'vertical' | 'horizontal' | 'grid';
}

export function KeyValueDisplay({
  items,
  layout = 'grid',
}: KeyValueDisplayProps) {
  const renderItem = (key: string, item: KeyValueItem) => {
    return (
      <div
        data-layout={layout}
        key={key}
        className='flex flex-col [[data-layout=horizontal]]:flex-row [[data-layout=horizontal]]:items-center [[data-layout=horizontal]]:gap-2'
      >
        <span className='text-gray-text-muted font-medium'>{item.key}:</span>
        <span className='text-gray-text'>{item.value}</span>
      </div>
    );
  };

  return (
    <div className={keyValueVariants({ layout })}>
      {items.map((item, index) => renderItem(index.toString(), item))}
    </div>
  );
}
