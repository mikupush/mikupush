import FileIcon from "@/components/FileIcon"
import { Large, Small } from "@/components/Typography"
import { Button } from "@/components/ui/button"
import { Progress } from "@/components/ui/progress"
import { extractExtension } from "@/helpers/file"
import { UploadRequest } from "@/model/upload"
import { LinkIcon, RotateCwIcon, TrashIcon, XIcon } from "lucide-react"
import { useEffect, useState } from "react"
import { listen } from "@tauri-apps/api/event"
import { ProgressEvent } from "@/model/events"
import { formatDate, formatRate, formatSizeBytes } from "@/helpers/format"
import { JSX } from "react/jsx-runtime"
import { invoke } from "@tauri-apps/api/core"
import { useTranslation } from "react-i18next"
import toast from 'react-hot-toast'

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
  const [uploadRequest, setUploadRequest] = useState(item)

  useEffect(() => {
    console.log('item', item)
    const progressListener = listen<ProgressEvent>(
      'upload-progress-changed',
      (event) => {
        const progress = event.payload

        if (progress.uploadId === item.upload.id) {
          setUploadRequest(previous => ({
            ...previous,
            progress: progress.progress,
            rateBytes: progress.rateBytes,
            uploadedBytes: progress.uploadedBytes
          }))
        }
      }
    )

    const finishListener = listen<UploadRequest>(
      'upload-finish',
      (event) => {
        const request = event.payload

        if (request.upload.id === item.upload.id) {
          console.log('finish fired', request)
          setUploadRequest(request)
        }
      }
    )

    return () => {
      progressListener.then(unlistenFn => unlistenFn())
      finishListener.then(unlistenFn => unlistenFn())
    }
  }, [])

  return (
    <UploadItemLayout
      item={item}
      body={<UploadProgressBody item={uploadRequest} />}
      actions={<UploadActions item={uploadRequest} />}
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
        {item.error ?? ''}
      </Small>
    )
  }

  console.log('render progress is progress')
  return (
    <>
      <Progress className="h-3 mt-[10px]" value={item.progress * 100}></Progress>
      <div className="flex place-content-between mt-[10px]">
        <Small>{formatRate(item.rateBytes)}</Small>
        <Small>{formatSizeBytes(item.uploadedBytes)} / {formatSizeBytes(item.upload.size)}</Small>
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
        <Button variant="outline" size="icon">
          <XIcon color="red" />
        </Button>
      </>
    )
  }

  return (
    <>
      <Button variant="outline" size="icon">
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
      <Button variant="outline" size="icon">
        <TrashIcon color="red" />
      </Button>
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
