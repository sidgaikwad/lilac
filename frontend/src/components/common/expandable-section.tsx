import { ChevronDown, ChevronRight } from 'lucide-react';
import { useState } from 'react';

export interface ExpandableSectionProps {
  className?: string;
  children?: React.ReactNode;
  title?: React.ReactNode;
}

export function ExpandableSection(props: ExpandableSectionProps) {
  const [isExpanded, setExpanded] = useState(false);
  const toggleExpanded = () => setExpanded(!isExpanded);
  return (
    <div className='flex flex-col space-y-2'>
      <button
        className='text-gray-text-muted flex flex-row items-center gap-x-2 text-sm'
        type='button'
        onClick={toggleExpanded}
      >
        {isExpanded ? (
          <ChevronDown className='size-5' />
        ) : (
          <ChevronRight className='size-5' />
        )}
        {props.title}
      </button>
      {isExpanded ? props.children : undefined}
    </div>
  );
}
