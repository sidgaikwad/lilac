import * as React from 'react';

export function SASIcon(props: React.SVGProps<SVGSVGElement>) {
  return (
    <svg
      {...props}
      xmlns='http://www.w3.org/2000/svg'
      width='1em'
      height='1em'
      viewBox='0 0 24 24'
    >
      <path
        fill='currentColor'
        d='M12 0C5.373 0 0 5.373 0 12s5.373 12 12 12s12-5.373 12-12S18.627 0 12 0zm0 22.5c-5.79 0-10.5-4.71-10.5-10.5S6.21 1.5 12 1.5S22.5 6.21 22.5 12S17.79 22.5 12 22.5z'
      />
      <path
        fill='currentColor'
        d='M12 6.2c-3.2 0-5.8 2.6-5.8 5.8s2.6 5.8 5.8 5.8s5.8-2.6 5.8-5.8s-2.6-5.8-5.8-5.8zm0 9.6c-2.1 0-3.8-1.7-3.8-3.8s1.7-3.8 3.8-3.8s3.8 1.7 3.8 3.8s-1.7 3.8-3.8 3.8z'
      />
    </svg>
  );
}