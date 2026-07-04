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

import { Button } from '@/components/ui/button.tsx'
import { useTranslation } from 'react-i18next'
import { OctagonXIcon, PlusIcon, SearchIcon } from 'lucide-react'
import ServerItem from '@/components/ServerItem.tsx'
import { useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { Server } from '@/model/server.ts'
import toast from 'react-hot-toast'
import { Paragraph } from '@/components/Typography.tsx'
import LoadingSpinner from '@/components/LoadingSpinner.tsx'
import { useServerConnector } from '@/hooks/server.ts'
import { useServer } from '@/context/ServerProvider.tsx'
import { useNavigate } from 'react-router'

export default function ServerListPage() {
  const { t } = useTranslation()
  const navigate = useNavigate()
  const [servers, setServers] = useState<Server[]>([])
  const [isLoading, setIsLoading] = useState(false)
  const { isConnecting, connectById } = useServerConnector()
  const { current } = useServer()

  useEffect(() => {
    setIsLoading(true)

    invoke<Server[]>('find_all_servers')
      .then(servers => setServers(servers))
      .catch(() => toast.error(t('errors.server.fetch_list')))
      .finally(() => setIsLoading(false))
  }, [])

  const deleteServer = (server: Server) => {
    invoke<void>('delete_server', { id: server.id })
      .then(() => {
        setServers(servers => servers.filter(item => item.id !== server.id))
        toast.success(t('server.delete.success'))
      })
      .catch(() => toast.error(t('server.delete.error')))
  }

  const connectServer = (server: Server) => {
    const connectPromise = connectById(server.id)
      .then((connectedServer) => {
        setServers(servers => servers.map(item => ({
          ...item,
          connected: item.id === server.id,
          healthy: item.id === server.id ? connectedServer.healthy : item.healthy
        })))
      })

    toast.promise(connectPromise, {
      loading: t('server.connect.loading'),
      success: t('server.connect.success'),
      error: (error) => typeof error === 'string' ? error : t('errors.unknown')
    })
  }

  const editServer = (server: Server) => {
    navigate(`/servers/${server.id}/edit`)
  }

  return (
    <div className="flex flex-1 flex-col py-3 pt-5 overflow-hidden">
      <div className="px-5 pb-5">
        <div className="flex justify-between w-full max-w-3xl mx-auto">
          <Button onClick={() => navigate('/servers/add')}>
            <PlusIcon />
            {t('server.add')}
          </Button>
          <Button variant="outline">
            <SearchIcon />
          </Button>
        </div>
      </div>
      {(isLoading) ? (
        <LoadingSpinner size={50} />
      ) : (servers.length > 0) ? (
        <div className="flex flex-1 overflow-y-auto px-5">
          <ul className="space-y-6 w-full max-w-3xl mx-auto pb-5">
            {servers.map((server: Server) => (
              <ServerItem
                key={server.id}
                {...server}
                disabled={isConnecting}
                connected={current.id === server.id}
                onConnect={() => connectServer(server)}
                onEdit={() => editServer(server)}
                onDelete={() => deleteServer(server)}
              />
            ))}
          </ul>
        </div>
      ) : (
        <div className="flex flex-1 flex-col justify-center items-center overflow-y-auto px-20">
          <OctagonXIcon className="size-15" />
          <Paragraph className="text-center">{t('server.empty')}</Paragraph>
        </div>
      )}
    </div>
  )
}
