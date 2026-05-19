import { Button } from '@/components/ui/button.tsx'
import { useTranslation } from 'react-i18next'
import { PlusIcon, SearchIcon } from 'lucide-react'
import ServerItem from '@/components/ServerItem.tsx'
import { useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { Server } from '@/model/server.ts'

export default function ServerListPage() {
  const { t } = useTranslation()
  const [servers, setServers] = useState<Server[]>([])

  useEffect(() => {
    invoke<Server[]>('find_all_servers')
      .then(servers => setServers(servers))
  }, [])

  return (
    <div className="flex flex-1 flex-col py-3 pt-5 overflow-hidden">
      <div className="px-5 pb-5">
        <div className="flex justify-between w-full max-w-3xl mx-auto">
          <Button>
            <PlusIcon />
            {t('server.add')}
          </Button>
          <Button variant="outline">
            <SearchIcon />
          </Button>
        </div>
      </div>
      <div className="flex flex-1 overflow-y-auto px-5">
        <ul className="space-y-6 w-full max-w-3xl mx-auto pb-5">
          {servers.map((server: Server) => (
            <ServerItem {...server} />
          ))}
        </ul>
      </div>
    </div>
  )
}