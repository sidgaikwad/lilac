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
        'group h-full w-full relative rounded-xl text-start flex-1 basis-0',
        'data-[state=checked]:ring-accent-border-hover data-[state=checked]:ring-2'
      )}
    >
      <div className="w-full h-full flex flex-col items-center justify-center">
        <Card
          title={title}
          description={description}
          icon={icon}
          className="w-full h-full items-center justify-center"
        />
      </div>
    </RadioGroupPrimitive.Item>
  );
}