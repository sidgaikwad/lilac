import * as React from 'react';

export function GPUIcon(props: React.SVGProps<SVGSVGElement>) {
  return (
    <svg
      {...props}
      xmlns='http://www.w3.org/2000/svg'
      width='1em'
      height='1em'
      viewBox='0 0 24 24'
    >
      <g
        fill='none'
        stroke='currentColor'
        strokeLinecap='round'
        strokeLinejoin='round'
        strokeWidth='2'
      >
        <path d='M18 9h-2V7a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v2H6a2 2 0 0 0-2 2v4a2 2 0 0 0 2 2h2v2a2 2 0 0 0 2 2h4a2 2 0 0 0 2-2v-2h2a2 2 0 0 0 2-2v-4a2 2 0 0 0-2-2' />
        <path d='M12 15a3 3 0 1 0 0-6a3 3 0 0 0 0 6' />
      </g>
    </svg>
  );
}