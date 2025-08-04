import { Eye, EyeOff } from 'lucide-react';
import { useState } from 'react';
import { Input } from '../ui/input';

export function Secret(props: { secret: string }) {
  const [isVisible, setIsVisible] = useState(false);

  const toggleVisibility = () => setIsVisible((prevState) => !prevState);
  return (
    <div className='relative'>
      <Input
        id='secret'
        type={isVisible ? 'text' : 'password'}
        aria-label='Secret'
        value={props.secret}
      />
      <button
        className='absolute inset-y-0 end-0 z-20 flex cursor-pointer items-center rounded-e-md px-2.5 text-gray-400 transition-colors hover:text-blue-500 focus:outline-none focus-visible:text-blue-500'
        type='button'
        onClick={toggleVisibility}
        aria-label={isVisible ? 'Hide secret' : 'Show secret'}
        aria-pressed={isVisible}
        aria-controls='secret'
      >
        {isVisible ? (
          <EyeOff size={20} aria-hidden='true' />
        ) : (
          <Eye size={20} aria-hidden='true' />
        )}
      </button>
    </div>
  );
}
