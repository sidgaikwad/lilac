import * as React from 'react';

export function CPUIcon(props: React.SVGProps<SVGSVGElement>) {
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
        <rect width='14' height='14' x='5' y='5' rx='2' />
        <path d='M9 9h6v6H9zM9 1v4m6-4v4m0 14v4m-6-4v4M1 9h4m14 0h4M1 15h4m14 0h4' />
      </g>
    </svg>
  );
}