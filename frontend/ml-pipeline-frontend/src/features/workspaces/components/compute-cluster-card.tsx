import { Card } from '@/components/common/card';
import { cn } from '@/lib/utils';
import * as RadioGroupPrimitive from '@radix-ui/react-radio-group';
import * as React from 'react';

interface ComputeClusterCardProps {
  icon: React.ReactNode;
  title: string;
  description: string;
  value: string;
}

export function ComputeClusterCard({
  icon,
  title,
  description,
  value,
}: ComputeClusterCardProps) {
  return (
    <RadioGroupPrimitive.Item
      key={value}
      value={value}
      className={cn(
        'group relative h-full w-full flex-1 basis-0 rounded-xl text-start',
        'data-[state=checked]:ring-accent-border-hover data-[state=checked]:ring-2'
      )}
    >
      <div className='flex h-full w-full flex-col items-center justify-center'>
        <Card
          title={title}
          description={description}
          icon={icon}
          className='h-full w-full items-center justify-center'
        />
      </div>
    </RadioGroupPrimitive.Item>
  );
}
