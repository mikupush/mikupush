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

import { undefinedServer, Server, ServerNotFoundError } from '@/model/server.ts'
import { createContext, ReactNode, useContext, useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import toast from 'react-hot-toast'
import { useTranslation } from 'react-i18next'

interface ServerContext {
  current: Server
  setCurrentById: (serverId: string) => Promise<Server>
  setCurrentByUrl: (url: string) => Promise<Server>
}

const ServerContext = createContext<ServerContext>({} as ServerContext)
export const useServer = () => useContext(ServerContext)

interface ServerProviderProps {
  children: ReactNode
}

export function ServerProvider({ children }: ServerProviderProps) {
  const [ current, setCurrent ] = useState<Server>(undefinedServer)
  const { t } = useTranslation()

  useEffect(() => {
    invoke<Server>('get_connected_server').then((server) => {
      setCurrent(server)
    }).catch((error) => {
      console.error('error getting connected server', error)
      toast.error(t('error.server.not_available'))
    })
  }, [])

  const setCurrentByUrl = async (url: string) => {
    let server = await invoke<Server | null>('get_server_by_url', { url: url })

    if (!server) {
      server = await invoke<Server>('create_server', {
        newServer: {
          alias: null,
          useAlias: false,
          url,
        }
      })
    }

    return await setCurrentById(server.id)
  }

  const setCurrentById = async (serverId: string) => {
    await invoke<void>('set_connected_server', { id: serverId })
    const server = await invoke<Server | null>('get_server_by_id', { id: serverId })

    if (!server) {
      throw new ServerNotFoundError(`server with id ${serverId} not found`)
    }

    const connectedServer = {
      ...server,
      connected: true,
      healthy: true
    }

    setCurrent(connectedServer)
    return connectedServer
  }

  const context = {
    current,
    setCurrentById,
    setCurrentByUrl
  }

  return (
    <ServerContext.Provider value={context}>
      {children}
    </ServerContext.Provider>
  )
}
