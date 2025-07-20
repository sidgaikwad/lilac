import { Skeleton } from '../ui/skeleton';

export function SkeletonCards() {
  return (
    <div className='flex h-full w-full flex-col gap-4 p-16'>
      <div className='flex max-h-14 grow flex-row gap-4'>
        <Skeleton className='grow' />
        <Skeleton className='grow' />
      </div>
      <div className='flex grow flex-row'>
        <Skeleton className='grow' />
      </div>
    </div>
  );
}
