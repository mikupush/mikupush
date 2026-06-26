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

import { LockIcon, PlugIcon, Settings2Icon, TrashIcon } from 'lucide-react'
import { Large, Small } from '@/components/Typography.tsx'
import { Button } from '@/components/ui/button.tsx'
import { Server } from '@/model/server.ts'
import {
  AlertDialog, AlertDialogAction, AlertDialogCancel,
  AlertDialogContent, AlertDialogDescription, AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger
} from '@/components/ui/alert-dialog.tsx'
import { useTranslation } from 'react-i18next'
import { ServerIcon } from '@/components/ServerIcon.tsx'

export interface ServerItemProps extends Server {
  restricted: boolean
  disabled: boolean
  onDelete: () => void
  onEdit: () => void
  onConnect: () => void
}

export default function ServerItem({
  name = 'Server name',
  url = 'https://example.com',
  icon,
  alias,
  useAlias = false,
  restricted = false,
  connected = false,
  disabled = false,
  onDelete = () => undefined,
  onEdit = () => undefined,
  onConnect = () => undefined,
}: Partial<ServerItemProps>) {
  const { t } = useTranslation()

  const displayName = (useAlias && alias != null && alias !== '') ? alias : name

  return (
    <li className="flex">
      <ServerIcon icon={icon} />
      <div className="flex flex-1 flex-col justify-between py-2 ml-3">
        <div className="flex items-center">
          <Large className="text-lg line-clamp-1 break-all max-w-4/6">{displayName}</Large>
          {restricted && <LockIcon className="ml-1 size-5" />}
          {connected && <PlugIcon className="text-green-500 ml-1 size-5" />}
        </div>
        <Small className="font-light text-lg line-clamp-1 break-all max-w-4/5">{url}</Small>
      </div>
      <div className="flex items-center">
        <Button variant="ghost" onClick={onConnect} disabled={disabled || connected}>
          <PlugIcon />
        </Button>
        <Button variant="ghost" onClick={onEdit} disabled={disabled}>
          <Settings2Icon />
        </Button>
        <AlertDialog>
          <AlertDialogTrigger asChild>
            <Button variant="ghost" disabled={disabled || connected}>
              <TrashIcon className="text-red-500" />
            </Button>
          </AlertDialogTrigger>
          <AlertDialogContent>
            <AlertDialogHeader>
              <AlertDialogTitle>{t('server.delete.confirmation.title')}</AlertDialogTitle>
              <AlertDialogDescription>{t('server.delete.confirmation.message')}</AlertDialogDescription>
            </AlertDialogHeader>
            <AlertDialogFooter>
              <AlertDialogCancel disabled={disabled}>{t('common.cancel')}</AlertDialogCancel>
              <AlertDialogAction variant="destructive" onClick={onDelete} disabled={disabled}>
                {t('common.delete')}
              </AlertDialogAction>
            </AlertDialogFooter>
          </AlertDialogContent>
        </AlertDialog>
      </div>
    </li>
  )
}
