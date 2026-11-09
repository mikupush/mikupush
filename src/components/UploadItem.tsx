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

import FileIcon from '@/components/FileIcon'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog'
import { extractExtension, uploadDisplayName } from '@/helpers/file'
import { formatDate, formatRate, formatSizeBytes } from '@/helpers/format'
import { Upload, UploadRequest } from '@/model/upload'
import { useUploadsStore } from '@/store/uploads'
import { invoke } from '@tauri-apps/api/core'
import { LinkIcon, LoaderCircle, RotateCwIcon, TrashIcon, XIcon } from 'lucide-react'
import toast from 'react-hot-toast'
import { useTranslation } from 'react-i18next'
import { ReactNode } from 'react'
import { cn } from '@/lib/utils'
import { cancelUpload, retryUpload } from '@/helpers/upload'

interface UploadItemProps {
  item: UploadRequest
}

export function UploadItem({ item }: UploadItemProps) {
  return <UploadProgressItem item={item} />
}

interface ArchivedUploadItemProps {
  upload: Upload
}

export function ArchivedUploadItem({ upload }: ArchivedUploadItemProps) {
  return <UploadRow upload={upload} actions={<ArchivedUploadActions upload={upload} />} />
}

export function UploadProgressItem({ item }: UploadItemProps) {
  return <UploadRow upload={item.upload} item={item} actions={<UploadActions item={item} />} />
}

interface UploadRowProps {
  upload: Upload
  item?: UploadRequest
  actions: ReactNode
}

function UploadRow({ upload, item, actions }: UploadRowProps) {
  const { t } = useTranslation()
  const displayName = uploadDisplayName(upload.name, upload.directory)
  const failed = item?.error != null || upload.status === 'failed'
  const canceled = item?.canceled || upload.status === 'aborted'
  const completed = !failed && !canceled && (item ? item.finished : upload.status === 'completed')
  const status = canceled ? 'aborted' : failed ? 'failed' : completed ? 'completed' : upload.status
  const transferring = status === 'inProgress' && !item?.finished
  const preparing = ['enqueued', 'pending', 'compressing'].includes(status)
  const fraction = preparing ? 0 : item?.progress.progress ?? 0
  const percentage = completed ? 100 : Number.isFinite(fraction) ? Math.min(99, Math.max(0, Math.floor(fraction * 100))) : 0
  const error = item?.error ? t(`errors.upload.${item.error.code}`, { message: item.error.message }) : undefined

  return (
    <tr className="transition-colors hover:bg-muted/50">
      <td className="px-3 py-4">
        <div className="flex min-w-0 items-center gap-3">
          <FileIcon extension={extractExtension(upload.name)} directory={upload.directory} />
          <span className="truncate font-medium" title={displayName}>{displayName}</span>
        </div>
      </td>
      <td className="px-3 py-4 text-right tabular-nums whitespace-nowrap">{formatSizeBytes(upload.size)}</td>
      <td className="px-3 py-4 text-right tabular-nums whitespace-nowrap text-muted-foreground">
        {transferring && item ? formatRate(item.progress.rateBytes) : '—'}
      </td>
      <td className="px-3 py-4">
        <div className={cn('flex items-center gap-2', (failed || canceled) && 'text-destructive', completed && 'text-file-spreadsheet')}>
          {status === 'compressing' && <LoaderCircle aria-hidden="true" className="size-4 shrink-0 animate-spin" />}
          <span>{t(`uploads.status.${status}`)}</span>
        </div>
        {error && <p className="mt-1 text-xs text-destructive break-words">{error}</p>}
      </td>
      <td className="px-3 py-4 text-right font-medium tabular-nums">{percentage}%</td>
      <td className="px-3 py-4 text-xs tabular-nums whitespace-nowrap text-muted-foreground">
        <time dateTime={upload.createdAt}>{formatDate(upload.createdAt)}</time>
      </td>
      <td className="px-2 py-4">
        <div className="flex items-center justify-end gap-1">{actions}</div>
      </td>
    </tr>
  )
}

function UploadActions({ item }: UploadItemProps) {
  const { t } = useTranslation()

  if (item.finished && item.error == null && !item.canceled && !['failed', 'aborted'].includes(item.upload.status)) {
    return <FinishedUploadActions item={item}/>
  }

  if (item.finished && item.error != null) {
    return (
      <>
        <Button
          variant="ghost"
          size="icon-sm"
          aria-label={t('uploads.actions.retry')}
          onClick={() => retryUpload(item.upload.id)}
        >
          <RotateCwIcon/>
        </Button>
        <Button
          variant="ghost"
          size="icon-sm"
          aria-label={t('uploads.actions.cancel')}
          onClick={() => cancelUpload(item.upload.id)}
        >
          <XIcon className="text-destructive"/>
        </Button>
      </>
    )
  }

  return (
    <>
      <Button
        variant="ghost"
        size="icon-sm"
        aria-label={t('uploads.actions.cancel')}
        onClick={() => cancelUpload(item.upload.id)}
      >
        <XIcon className="text-destructive"/>
      </Button>
    </>
  )
}

function FinishedUploadActions({ item }: UploadItemProps) {
  const { t } = useTranslation()

  return (
    <>
      <Button
        aria-label={t('uploads.actions.copy_link')}
        onClick={() => {
          invoke('copy_upload_link', { uploadId: item.upload.id })
            .then(() => toast.success(t('uploads.link_copied.success')))
            .catch(() => toast.error(t('uploads.link_copied.error')))
        }}
        variant="ghost"
        size="icon-sm"
      >
        <LinkIcon/>
      </Button>
      <DeleteAction item={item}/>
    </>
  )
}

function ArchivedUploadActions({ upload }: ArchivedUploadItemProps) {
  const { t } = useTranslation()

  return (
    <>
      <Button
        aria-label={t('uploads.actions.copy_link')}
        onClick={() => {
          invoke('copy_archived_upload_link', { uploadId: upload.id })
            .then(() => toast.success(t('uploads.link_copied.success')))
            .catch(() => toast.error(t('uploads.link_copied.error')))
        }}
        variant="ghost"
        size="icon-sm"
      >
        <LinkIcon/>
      </Button>
      <DeleteArchivedAction upload={upload}/>
    </>
  )
}

function DeleteAction({ item }: UploadItemProps) {
  const { t } = useTranslation()
  const { setInProgressUploads } = useUploadsStore()
  const displayName = uploadDisplayName(item.upload.name, item.upload.directory)

  const performDelete = () => {
    invoke<UploadRequest[]>('delete_upload', { uploadId: item.upload.id })
      .then((uploadsRequests) => {
        toast.success(t('uploads.delete.success', { fileName: displayName }))
        setInProgressUploads(uploadsRequests)
      })
      .catch(() => toast.error(t('uploads.delete.error', { fileName: displayName })))
  }

  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button variant="ghost" size="icon-sm" aria-label={t('uploads.actions.delete')}>
          <TrashIcon className="text-destructive"/>
        </Button>
      </DialogTrigger>
      <DialogContent className="sm:max-w-[425px]">
        <DialogHeader>
          <DialogTitle>{t('dialog.heading.danger')}</DialogTitle>
          <DialogDescription>{t('uploads.delete.warning')}</DialogDescription>
        </DialogHeader>
        <DialogFooter>
          <DialogClose asChild>
            <Button variant="outline">{t('uploads.delete.cancel')}</Button>
          </DialogClose>
          <DialogClose asChild>
            <Button variant="destructive" onClick={performDelete}>{t('uploads.delete.confirm')}</Button>
          </DialogClose>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}

function DeleteArchivedAction({ upload }: ArchivedUploadItemProps) {
  const { t } = useTranslation()
  const { setArchivedUploads } = useUploadsStore()
  const displayName = uploadDisplayName(upload.name, upload.directory)

  const performDelete = () => {
    invoke<Upload[]>('delete_archived_upload', { uploadId: upload.id })
      .then((uploads) => {
        toast.success(t('uploads.delete.success', { fileName: displayName }))
        setArchivedUploads(uploads)
      })
      .catch(() => toast.error(t('uploads.delete.error', { fileName: displayName })))
  }

  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button variant="ghost" size="icon-sm" aria-label={t('uploads.actions.delete')}>
          <TrashIcon className="text-destructive"/>
        </Button>
      </DialogTrigger>
      <DialogContent className="sm:max-w-[425px]">
        <DialogHeader>
          <DialogTitle>{t('dialog.heading.danger')}</DialogTitle>
          <DialogDescription>{t('uploads.delete.warning')}</DialogDescription>
        </DialogHeader>
        <DialogFooter>
          <DialogClose asChild>
            <Button variant="outline">{t('uploads.delete.cancel')}</Button>
          </DialogClose>
          <DialogClose asChild>
            <Button variant="destructive" onClick={performDelete}>{t('uploads.delete.confirm')}</Button>
          </DialogClose>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
