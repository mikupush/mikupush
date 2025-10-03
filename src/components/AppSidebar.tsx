import {
  Sidebar,
  SidebarContent, SidebarGroup, SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from '@/components/ui/sidebar.tsx'
import { SettingsIcon, UploadIcon } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { NavLink } from 'react-router'
import SelectedServer from '@/components/SelectedServer.tsx'
import { platform } from '@tauri-apps/plugin-os'
import SidebarLinkMenuItem from '@/components/SidebarLinkMenuItem.tsx'

export default function AppSidebar() {
  const { t } = useTranslation()

  return (
    <Sidebar variant="sidebar" collapsible="offcanvas">
      <SidebarHeader className={platform() === 'macos' ? 'mt-10': ''}>
        <SelectedServer/>
      </SidebarHeader>
      <SidebarContent>
        <SidebarGroup>
          <SidebarMenu>
            <SidebarLinkMenuItem to="/">
              <UploadIcon />
              <span>{t('sidebar.upload')}</span>
            </SidebarLinkMenuItem>
            <SidebarLinkMenuItem to="/settings">
              <SettingsIcon />
              <span>{t('sidebar.settings')}</span>
            </SidebarLinkMenuItem>
          </SidebarMenu>
        </SidebarGroup>
      </SidebarContent>
    </Sidebar>
  )
}