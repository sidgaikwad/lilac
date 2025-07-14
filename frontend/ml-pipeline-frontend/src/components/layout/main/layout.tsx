import React from 'react';
import { Outlet } from 'react-router-dom';
import { Skeleton } from '@/components/ui/skeleton';
import { Toaster } from '@/components/ui/sonner';
import Header from './header';
import Footer from './footer';

export default function Layout() {
  return (
    <div className='[--header-height:calc(theme(spacing.14))]'>
      <Header />

      <div className='flex flex-1'>
        <div className='@container flex flex-1 flex-row md:flex-col'>
          <Toaster position='top-center' richColors closeButton />
          <React.Suspense fallback={<Skeleton className='h-full w-full' />}>
            <Outlet />
          </React.Suspense>
        </div>
      </div>

      <footer>
        <Footer />
      </footer>
    </div>
  );
}
