import { createServerFromUrl, undefinedServer, Server, ServerNotFoundError } from '@/model/server.ts'
import { createContext, ReactNode, useContext, useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import toast from 'react-hot-toast'
import { useTranslation } from 'react-i18next'

interface ServerContext {
  current: Server
  setCurrentById: (serverId: string) => Promise<void>
  setCurrentByUrl: (url: string) => Promise<void>
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
        new_server: createServerFromUrl(url)
      })
    }

    await invoke<Server>('set_connected_server', { id: server.id })
    setCurrent(server)
  }

  const setCurrentById = async (serverId: string) => {
    await invoke<Server>('set_connected_server', { id: serverId })
    const server = await invoke<Server | null>('get_server_by_id', { id: serverId })

    if (!server) {
      throw new ServerNotFoundError(`server with id ${serverId} not found`)
    }

    setCurrent(server)
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