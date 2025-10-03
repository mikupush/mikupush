import { Outlet } from 'react-router'
import AppSidebar from '@/components/AppSidebar.tsx'
import { SidebarInset, SidebarProvider } from '@/components/ui/sidebar.tsx'
import AppTitleBar from '@/components/AppTitleBar.tsx'

export default function DefaultLayout() {
  return (
    <>
      <SidebarProvider>
        <AppSidebar />
        <SidebarInset>
          <div className="flex flex-1 flex-col">
            <AppTitleBar />
            <main>
              <Outlet />
            </main>
          </div>
        </SidebarInset>
      </SidebarProvider>
    </>
  )
}