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
import { useTranslation } from 'react-i18next'
import { NavLink } from 'react-router'
import { ServerIcon } from '@/components/ServerIcon.tsx'

export function SelectedServerSidebarMenu() {
  const { isMobile } = useSidebar()
  const { current } = useServer()
  const name = current.useAlias && current.alias != null && current.alias !== ''
    ? current.alias
    : current.name

  return (
    <SidebarMenu>
      <SidebarMenuItem>
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <SidebarMenuButton
              size="lg"
              className="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
            >
              <ServerIcon className="size-8" icon={current.icon} />
              <div className="grid flex-1 text-left text-sm leading-tight">
                <span className="truncate font-medium">{name}</span>
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
  const name = current.useAlias && current.alias ? current.alias : current.name

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button variant="ghost">
          <ServerIcon className="size-full" icon={current.icon} />
          <Small>{name}</Small>
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
      <DropdownMenuItem asChild className="gap-2 p-2">
        <NavLink to="/servers">
          <div className="flex size-6 items-center justify-center bg-transparent">
            <Server className="size-4" />
          </div>
          <div className="font-medium">{t('server.manage')}</div>
        </NavLink>
      </DropdownMenuItem>
    </DropdownMenuContent>
  )
}
