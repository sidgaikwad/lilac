import { cva, VariantProps } from 'class-variance-authority';
import './spinner.css';
import { cn } from '@/lib';

const spinnerVariants = cva('rotate', {
  variants: {
    size: {
      small: 'size-6',
      medium: 'size-8',
      large: 'size-12',
      xlarge: 'size-16',
    },
  },
  defaultVariants: {
    size: 'small',
  },
});

export function Spinner(
  props: VariantProps<typeof spinnerVariants> & { className?: string }
) {
  return (
    <svg
      className={cn(spinnerVariants({ size: props.size }), props.className)}
      viewBox='0 0 100 100'
      xmlns='http://www.w3.org/2000/svg'
    >
      <circle
        className='spinner'
        cx='50'
        cy='50'
        fill='none'
        r='46'
        stroke-width='8'
        stroke='var(--color-gray-400)'
        opacity='70%'
        transform-origin='50% 50%'
        stroke-dasharray='289'
        stroke-linecap='round'
      />
    </svg>
  );
}
