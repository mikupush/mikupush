import { LockIcon, PlugIcon, ServerIcon, Settings2Icon, TrashIcon } from 'lucide-react'
import { Large, Small } from '@/components/Typography.tsx'
import { Button } from '@/components/ui/button.tsx'
import { Server } from '@/model/server.ts'

export interface ServerItemProps extends Server {
  restricted: boolean
  onDelete: () => void
  onEdit: () => void
  onConnect: () => void
}

export default function ServerItem({
  name = 'Server name',
  url = 'https://example.com',
  icon,
  restricted = false,
  connected = false,
  onDelete = () => undefined,
  onEdit = () => undefined,
  onConnect = () => undefined,
}: Partial<ServerItemProps>) {
  return (
    <li className="flex">
      <div className="size-20 flex justify-center items-center rounded-lg overflow-hidden">
        {icon ? (
          <img className="h-full object-cover" src={icon} alt="" />
        ) : (
          <ServerIcon className="size-20" />
        )}
      </div>
      <div className="flex flex-1 flex-col justify-between py-2 ml-3">
        <div className="flex items-center">
          <Large className="text-lg line-clamp-1 break-all max-w-4/6">{name}</Large>
          {restricted && <LockIcon className="ml-1 size-5" />}
          {connected && <PlugIcon className="text-green-500 ml-1 size-5" />}
        </div>
        <Small className="font-light text-lg line-clamp-1 break-all max-w-4/5">{url}</Small>
      </div>
      <div className="flex items-center">
        <Button variant="ghost" onClick={onConnect}>
          <PlugIcon />
        </Button>
        <Button variant="ghost" onClick={onEdit}>
          <Settings2Icon />
        </Button>
        <Button variant="ghost" onClick={onDelete}>
          <TrashIcon className="text-red-500" />
        </Button>
      </div>
    </li>
  )
}