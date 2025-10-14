/**
 * Copyright 2025 Miku Push! Team
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

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