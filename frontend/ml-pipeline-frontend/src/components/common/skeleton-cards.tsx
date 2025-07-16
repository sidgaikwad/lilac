import { Skeleton } from "../ui/skeleton";


export function SkeletonCards() {
  return (
    <div className='p-16 w-full h-full flex flex-col gap-4'>
      <div className='grow flex flex-row gap-4 max-h-14'>
        <Skeleton className='grow' />
        <Skeleton className='grow' />
      </div>
      <div className='grow flex flex-row'>
        <Skeleton className='grow' />
      </div>
    </div>
  );
}