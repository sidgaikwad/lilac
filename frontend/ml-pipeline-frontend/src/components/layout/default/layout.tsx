import React from 'react';
import { Outlet } from 'react-router-dom';
import { Toaster } from '@/components/ui/sonner';
import Header from './header';
import Footer from './footer';
import { SkeletonCards } from '@/components/common/skeleton-cards';

export default function Layout() {
  return (
    <div className='[--header-height:calc(theme(spacing.14))]'>
      <Header />

      <div className='flex flex-1'>
        <div className='@container flex flex-1 flex-row md:flex-col'>
          <Toaster position='top-center' richColors closeButton />
          <React.Suspense fallback={<SkeletonCards />}>
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
