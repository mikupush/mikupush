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

import { SidebarMenu, SidebarMenuButton, SidebarMenuItem, useSidebar } from '@/components/ui/sidebar'
import {
  DropdownMenu,
  DropdownMenuContent, DropdownMenuItem,
  DropdownMenuLabel, DropdownMenuSeparator,
  DropdownMenuTrigger
} from '@/components/ui/dropdown-menu'
import { Button } from '@/components/ui/button'
import { ChevronsUpDown, Server } from 'lucide-react'
import { Small } from '@/components/Typography.tsx'
import { useServer } from '@/context/ServerProvider.tsx'
import { useServerIcon } from '@/hooks/server.ts'
import { useTranslation } from 'react-i18next'

export function SelectedServerSidebarMenu() {
  const { isMobile } = useSidebar()
  const { current } = useServer()
  const icon = useServerIcon(current)

  return (
    <SidebarMenu>
      <SidebarMenuItem>
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <SidebarMenuButton
              size="lg"
              className="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
            >
              <div className="flex aspect-square size-8 items-center justify-center rounded-lg">
                <img className="h-full" src={icon} alt="" />
              </div>
              <div className="grid flex-1 text-left text-sm leading-tight">
                <span className="truncate font-medium">{current.alias ?? current.name}</span>
                {/*<span className="truncate text-xs">Premium</span>*/}
              </div>
              <ChevronsUpDown className="ml-auto" />
            </SidebarMenuButton>
          </DropdownMenuTrigger>
          <DropdownMenuItems side={isMobile ? 'bottom' : 'right'} />
        </DropdownMenu>
      </SidebarMenuItem>
    </SidebarMenu>
  )
}

export function SelectedServerDropdown() {
  const { current } = useServer()
  const icon = useServerIcon(current)

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button variant="ghost">
          <img className="h-full" src={icon} alt="" />
          <Small>{current.alias ?? current.name}</Small>
          <ChevronsUpDown className="ml-auto" />
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuItems />
    </DropdownMenu>
  )
}

interface DropdownProps {
  side?: 'bottom' | 'right'
}

function DropdownMenuItems({ side = 'bottom' }: DropdownProps) {
  const { t } = useTranslation()

  return (
    <DropdownMenuContent
      className="w-(--radix-dropdown-menu-trigger-width) min-w-56 rounded-lg"
      align="start"
      side={side}
      sideOffset={4}
    >
      <DropdownMenuLabel className="text-muted-foreground text-xs">
        {t('server.recent')} 🚧
      </DropdownMenuLabel>
      <DropdownMenuItem
        className="gap-2 p-2"
      >
        <div className="flex size-6 items-center justify-center rounded-md">
          <Server className="size-4" />
        </div>
        Example server
      </DropdownMenuItem>
      <DropdownMenuSeparator />
      <DropdownMenuItem className="gap-2 p-2">
        <div className="flex size-6 items-center justify-center bg-transparent">
          <Server className="size-4" />
        </div>
        <div className="font-medium">{t('server.manage')} 🚧</div>
      </DropdownMenuItem>
    </DropdownMenuContent>
  )
}