import { Card } from '@/components/common/card';
import { cn } from '@/lib/utils';
import * as RadioGroupPrimitive from '@radix-ui/react-radio-group';
import * as React from 'react';

interface EnvironmentCardProps {
  icon: React.ReactNode;
  title: string;
  description: string;
  value: string;
  className?: string;
  layout?: 'horizontal' | 'vertical';
}

export function EnvironmentCard({
  icon,
  title,
  description,
  value,
  className,
  layout,
}: EnvironmentCardProps) {
  return (
    <RadioGroupPrimitive.Item
      key={value}
      value={value}
      className={cn(
        'group relative rounded-xl text-start',
        'data-[state=checked]:ring-accent-border-hover h-full w-full data-[state=checked]:ring-2'
      )}
    >
      <Card
        icon={icon}
        title={title}
        description={description}
        className={cn('h-full w-full', className)}
        layout={layout}
      />
    </RadioGroupPrimitive.Item>
  );
}
