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
import LoadingSpinner from '@/components/LoadingSpinner.tsx'
import { Paragraph } from '@/components/Typography.tsx'
import { ArchivedUploadList } from '@/components/UploadList.tsx'
import { Button } from '@/components/ui/button'
import { useServerConnector } from '@/hooks/server.ts'
import { Upload } from '@/model/upload.ts'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { ArchiveIcon, LoaderCircle, RefreshCwIcon } from 'lucide-react'
import { useEffect, useState } from 'react'
import toast from 'react-hot-toast'
import { useTranslation } from 'react-i18next'

export default function CompletedUploadsPage() {
  const { current } = useServerConnector()
  const { t } = useTranslation()
  const [history, setHistory] = useState<{ serverId: string, uploads: Upload[] }>()
  const [isSyncing, setIsSyncing] = useState(false)
  const archivedUploads = history?.serverId === current.id ? history?.uploads : undefined

  useEffect(() => {
    const serverId = current.id
    if (!serverId) return

    let disposed = false
    let revision = 0
    let unlisten: UnlistenFn | undefined

    const refresh = async () => {
      const request = ++revision
      try {
        const uploads = await invoke<Upload[]>('get_archived_uploads', { serverId })
        if (!disposed && request === revision) setHistory({ serverId, uploads })
      } catch (error) {
        console.warn('error getting archived uploads', error)
        if (!disposed && request === revision) {
          setHistory(previous => previous?.serverId === serverId ? previous : { serverId, uploads: [] })
        }
      }
    }

    listen<string>('archived-uploads-changed', ({ payload }) => {
      if (!disposed && payload === serverId) void refresh()
    })
      .then((stopListening) => {
        if (disposed) {
          stopListening()
          return
        }
        unlisten = stopListening
      })
      .catch(error => console.warn('error listening for archived upload changes', error))
      .finally(() => {
        if (!disposed) void refresh()
      })

    invoke('set_archived_uploads_view', { serverId })
      .catch(error => console.warn('error opening archived uploads view', error))

    return () => {
      disposed = true
      unlisten?.()
      invoke('set_archived_uploads_view', { serverId: null })
        .catch(error => console.warn('error closing archived uploads view', error))
    }
  }, [current.id])

  const syncStatuses = async () => {
    setIsSyncing(true)
    try {
      const ran = await invoke<boolean>('sync_archived_upload_statuses', { serverId: current.id })
      if (!ran) toast(t('uploads.status_sync.in_progress'))
    } catch (error) {
      console.warn('error syncing archived upload statuses', error)
      toast.error(t('uploads.status_sync.error'))
    } finally {
      setIsSyncing(false)
    }
  }

  return (
    <div className="flex min-w-0 flex-1">
      {current.id && !archivedUploads ? (
        <LoadingSpinner size={50} />
      ) : archivedUploads?.length ? (
        <ArchivedUploadList
          items={archivedUploads}
          syncAction={(
            <Button type="button" variant="ghost" size="sm" onClick={syncStatuses} disabled={isSyncing} aria-busy={isSyncing}>
              {isSyncing
                ? <LoaderCircle className="animate-spin" data-icon="inline-start" />
                : <RefreshCwIcon data-icon="inline-start" />}
              {t('uploads.status_sync.action')}
            </Button>
          )}
        />
      ) : (
        <EmptyState />
      )}
    </div>
  )
}

function EmptyState() {
  const { t } = useTranslation()

  return (
    <div className="flex flex-1 flex-col justify-center items-center px-20">
      <ArchiveIcon width={60} height={60} />
      <Paragraph className="text-center">{t('uploads.completed_empty_state')}</Paragraph>
    </div>
  )
}
