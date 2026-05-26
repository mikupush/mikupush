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

export default function ServerListPage() {
  const { t } = useTranslation()
  const [servers, setServers] = useState<Server[]>([])
  const [isLoading, setIsLoading] = useState(false)

  useEffect(() => {
    setIsLoading(true)

    invoke<Server[]>('find_all_servers')
      .then(servers => setServers(servers))
      .catch(() => toast.error(t('errors.server.fetch_list')))
      .finally(() => setIsLoading(false))
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
      {(isLoading) ? (
        <LoadingSpinner size={50} />
      ) : (servers.length > 0) ? (
        <div className="flex flex-1 overflow-y-auto px-5">
          <ul className="space-y-6 w-full max-w-3xl mx-auto pb-5">
            {servers.map((server: Server) => (
              <ServerItem {...server} />
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