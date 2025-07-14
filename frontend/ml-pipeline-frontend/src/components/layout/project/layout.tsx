import React from 'react';
import { Outlet } from 'react-router-dom';
import { SidebarInset, SidebarProvider } from '@/components/ui/sidebar';
import { Skeleton } from '@/components/ui/skeleton';
import { Toaster } from '@/components/ui/sonner';
import Sidebar from './sidebar';
import Header from './header';
import Footer from '../main/footer';

export default function Layout() {
  return (
    <div className='[--header-height:calc(theme(spacing.14))]'>
      <SidebarProvider className='flex flex-col' defaultOpen={false}>
        <Header />

        <div className='flex'>
          <Sidebar />
          <SidebarInset>
            <div className='@container flex flex-1 flex-row md:flex-col'>
              <Toaster position='top-center' richColors closeButton />
              <React.Suspense fallback={<Skeleton className='h-full w-full' />}>
                <Outlet />
              </React.Suspense>
            </div>
          </SidebarInset>
        </div>

        <footer>
          <Footer />
        </footer>
      </SidebarProvider>
    </div>
  );
}
