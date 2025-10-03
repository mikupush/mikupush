import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarHeader,
  SidebarMenu,
} from '@/components/ui/sidebar.tsx'
import { ArchiveIcon, SettingsIcon, UploadIcon } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { SelectedServerSidebarMenu } from '@/components/SelectedServer.tsx'
import { platform } from '@tauri-apps/plugin-os'
import SidebarLinkMenuItem from '@/components/SidebarLinkMenuItem.tsx'

export default function AppSidebar() {
  const { t } = useTranslation()

  return (
    <Sidebar variant="sidebar" collapsible="offcanvas">
      <SidebarHeader className={platform() === 'macos' ? 'mt-10': ''}>
        <SelectedServerSidebarMenu />
      </SidebarHeader>
      <SidebarContent>
        <SidebarGroup>
          <SidebarMenu>
            <SidebarLinkMenuItem to="/">
              <UploadIcon />
              <span>{t('sidebar.uploads')}</span>
            </SidebarLinkMenuItem>
            <SidebarLinkMenuItem to="/archived">
              <ArchiveIcon />
              <span>{t('sidebar.archived_uploads')}</span>
            </SidebarLinkMenuItem>
          </SidebarMenu>
        </SidebarGroup>
      </SidebarContent>
      <SidebarFooter>
        <SidebarGroup>
          <SidebarMenu>
            <SidebarLinkMenuItem to="/settings">
              <SettingsIcon />
              <span>{t('sidebar.settings')}</span>
            </SidebarLinkMenuItem>
          </SidebarMenu>
        </SidebarGroup>
      </SidebarFooter>
    </Sidebar>
  )
}