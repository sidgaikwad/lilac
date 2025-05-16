import React from 'react';
import { Outlet } from 'react-router-dom';
import { SidebarInset, SidebarProvider } from '@/components/ui/sidebar';
import { Skeleton } from '@/components/ui/skeleton';
import { Toaster } from '@/components/ui/toast';
import Sidebar from './sidebar';
import Header from './header';
import Footer from './footer';

export default function Layout() {
  return (
    <div className="[--header-height:calc(theme(spacing.14))]">
      <SidebarProvider className="flex flex-col" defaultOpen={false}>
        <Header />

        <div className="flex flex-1">
          <Sidebar />
          <SidebarInset>
            <div className="flex flex-1 flex-col gap-4 p-4 ml-8">
              <Toaster position="top-center" richColors closeButton />
              <React.Suspense fallback={<Skeleton className="w-full h-full" />}>
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
