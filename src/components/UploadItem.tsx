import FileIcon from "@/components/FileIcon"
import { Large, Small } from "@/components/Typography"
import { Button } from "@/components/ui/button"
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
import { Progress } from "@/components/ui/progress"
import { extractExtension } from "@/helpers/file"
import { formatDate, formatRate, formatSizeBytes } from "@/helpers/format"
import { UploadRequest } from "@/model/upload"
import { useUploadsStore } from "@/store/uploads"
import { invoke } from "@tauri-apps/api/core"
import { LinkIcon, RotateCwIcon, TrashIcon, XIcon } from "lucide-react"
import toast from 'react-hot-toast'
import { useTranslation } from "react-i18next"
import { JSX } from "react/jsx-runtime"
import { cancelUpload } from "@/helpers/upload.ts"

interface UploadItemProps {
  item: UploadRequest
}

export function UploadItem({ item }: UploadItemProps) {
  return (
    <UploadItemLayout
      item={item}
      body={<FinishedUploadBody item={item} />}
      actions={<FinishedUploadActions item={item} />}
    />
  )
}

interface UploadProgressProps {
  item: UploadRequest
}

export function UploadProgressItem({ item }: UploadProgressProps) {
  return (
    <UploadItemLayout
      item={item}
      body={<UploadProgressBody item={item} />}
      actions={<UploadActions item={item} />}
    />
  )
}

function UploadProgressBody({ item }: UploadItemProps) {
  if (item.finished && item.error == null) {
    return <FinishedUploadBody item={item} />
  }

  if (item.finished && item.error != null && item.error != '') {
    return (
      <Small className="mt-[10px] text-red-600 line-clamp-1">
        {item.error}
      </Small>
    )
  }

  return (
    <>
      <Progress className="h-3 mt-[10px]" value={item.progress.progress * 100}></Progress>
      <div className="flex place-content-between mt-[10px]">
        <Small>{formatRate(item.progress.rateBytes)}</Small>
        <Small>{formatSizeBytes(item.progress.uploadedBytes)} / {formatSizeBytes(item.upload.size)}</Small>
      </div>
    </>
  )
}

function FinishedUploadBody({ item }: UploadItemProps) {
  return (
    <Small className="mt-[10px] line-clamp-1">
      {formatSizeBytes(item.upload.size)} · {formatDate(item.upload.createdAt)}
    </Small>
  )
}

function UploadActions({ item }: UploadItemProps) {
  if (item.finished && item.error == null) {
    return <FinishedUploadActions item={item} />
  }

  if (item.finished && item.error != null && item.error != '') {
    return (
      <>
        <Button variant="outline" size="icon">
          <RotateCwIcon />
        </Button>
        <Button
            variant="outline"
            size="icon"
            onClick={() => cancelUpload(item.upload.id)}
        >
          <XIcon color="red" />
        </Button>
      </>
    )
  }

  return (
    <>
      <Button
          variant="outline"
          size="icon"
          onClick={() => cancelUpload(item.upload.id)}
      >
        <XIcon color="red" />
      </Button>
    </>
  )
}

function FinishedUploadActions({ item }: UploadItemProps) {
  const { t } = useTranslation()

  return (
    <>
      <Button
        onClick={() => {
          invoke('copy_upload_link', { uploadId: item.upload.id })
            .then(() => toast.success(t('uploads.link_copied.success')))
            .catch(() => toast.error(t('uploads.link_copied.error')))
        }}
        variant="outline"
        size="icon"
      >
        <LinkIcon />
      </Button>
      <DeleteAction item={item} />
    </>
  )
}

interface UploadItemLayout extends UploadItemProps {
  body: JSX.Element
  actions: JSX.Element
}

function UploadItemLayout({ body, actions, item }: UploadItemLayout) {
  return (
    <div className="flex p-[10px]">
      <FileIcon extension={extractExtension(item.upload.name)} />
      <div className="flex flex-1 flex-col mx-[10px]">
        <Large className="line-clamp-1">{item.upload.name}</Large>
        {body}
      </div>
      <div className="flex items-center space-x-[10px]">
        {actions}
      </div>
    </div>
  )
}

function DeleteAction({ item }: UploadItemProps) {
  const { t } = useTranslation()
  const { setInProgressUploads } = useUploadsStore()

  const performDelete = () => {
    invoke<UploadRequest[]>('delete_upload', { uploadId: item.upload.id })
      .then((uploadsRequests) => {
        toast.success(t('uploads.delete.success', { fileName: item.upload.name }))
        setInProgressUploads(uploadsRequests)
      })
      .catch(() => toast.error(t('uploads.delete.error', { fileName: item.upload.name })))
  }

  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button variant="outline" size="icon">
          <TrashIcon color="red" />
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