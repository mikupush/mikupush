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
import { ChevronsUpDown, Server as ServerIconLucide } from 'lucide-react'
import { Small } from '@/components/Typography.tsx'
import { useServer } from '@/context/ServerProvider.tsx'
import { useTranslation } from 'react-i18next'
import { NavLink } from 'react-router'
import { ServerIcon } from '@/components/ServerIcon.tsx'
import { useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { Server } from '@/model/server.ts'
import toast from 'react-hot-toast'

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
  const { current, setCurrentById } = useServer()
  const [recentServers, setRecentServers] = useState<Server[]>([])

  useEffect(() => {
    invoke<Server[]>('find_recent_servers')
      .then(setRecentServers)
      .catch((error) => {
        console.error('error getting recent servers', error)
      })
  }, [current.id])

  const selectServer = (server: Server) => {
    const connectPromise = setCurrentById(server.id)
      .then(() => invoke<Server[]>('find_recent_servers'))
      .then(setRecentServers)

    toast.promise(connectPromise, {
      loading: t('server.connect.loading'),
      success: t('server.connect.success'),
      error: (error) => typeof error === 'string' ? error : t('errors.unknown')
    })
  }

  return (
    <DropdownMenuContent
      className="w-(--radix-dropdown-menu-trigger-width) min-w-56 rounded-lg"
      align="start"
      side={side}
      sideOffset={4}
    >
      {recentServers.length > 0 && (
        <>
          <DropdownMenuLabel className="text-muted-foreground text-xs">
            {t('server.recent')}
          </DropdownMenuLabel>
          {recentServers.map((server) => {
            const name = server.useAlias && server.alias ? server.alias : server.name

            return (
              <DropdownMenuItem
                key={server.id}
                className="gap-2 p-2"
                disabled={server.id === current.id}
                onSelect={() => selectServer(server)}
              >
                <ServerIcon className="size-6" icon={server.icon} />
                <div className="flex min-w-0 flex-col">
                  <span className="truncate font-medium">{name}</span>
                  <span className="text-muted-foreground truncate text-xs">{server.url}</span>
                </div>
              </DropdownMenuItem>
            )
          })}
          <DropdownMenuSeparator />
        </>
      )}
      <DropdownMenuItem asChild className="gap-2 p-2">
        <NavLink to="/servers">
          <div className="flex size-6 items-center justify-center bg-transparent">
            <ServerIconLucide className="size-4" />
          </div>
          <div className="font-medium">{t('server.manage')}</div>
        </NavLink>
      </DropdownMenuItem>
    </DropdownMenuContent>
  )
}
