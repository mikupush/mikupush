/**
 * Miku Push! is a simple, lightweight, and open-source WeTransfer alternative for desktop.
 * Copyright (C) 2025  Miku Push! Team
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 * 
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarHeader,
  SidebarMenu,
} from '@/components/ui/sidebar.tsx'
import { ArchiveIcon, InfoIcon, SettingsIcon, UploadIcon } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { SelectedServerSidebarMenu } from '@/components/SelectedServer.tsx'
import { platform } from '@tauri-apps/plugin-os'
import SidebarLinkMenuItem from '@/components/SidebarLinkMenuItem.tsx'
import SidebarButtonMenuItem from '@/components/SidebarButtonMenuItem.tsx'
import { openAboutWindow } from '@/helpers/open.ts'

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
            <SidebarButtonMenuItem onClick={() => openAboutWindow()}>
              <InfoIcon />
              <span>{t('sidebar.about')}</span>
            </SidebarButtonMenuItem>
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