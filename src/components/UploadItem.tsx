import FileIcon from "@/components/FileIcon"
import { Large, Small } from "@/components/Typography"
import { Button } from "@/components/ui/button"
import { Progress } from "@/components/ui/progress"
import { extractExtension } from "@/helpers/file"
import { UploadRequest } from "@/model/upload"
import { XIcon } from "lucide-react"
import { useEffect, useState } from "react"
import { listen } from "@tauri-apps/api/event"
import { ProgressEvent } from "@/model/events"
import { formatRate, formatSizeBytes } from "@/helpers/format"

interface UploadItemProps {
  item: UploadRequest
}

export default function UploadItem({ item }: UploadItemProps) {
  return (
    <div className="flex p-[10px]">
      <FileIcon extension={extractExtension(item.upload.name)} />
      <div className="flex flex-1 flex-col place-content-between mx-[10px]">
        <Large>{item.upload.name}</Large>
        <UploadProgress item={item} />
      </div>
      <div className="flex items-center">
        <UploadActions />
      </div>
    </div>
  )
}

interface UploadProgressProps {
  item: UploadRequest
}

export function UploadProgress({ item }: UploadProgressProps) {
  const [progress, setProgress] = useState(item.progress)
  const [rate, setRate] = useState(item.rateBytes)
  const [uploaded, setUploaded] = useState(item.uploadedBytes)

  useEffect(() => {
    const progressListener = listen<ProgressEvent>(
      'upload-progress-changed',
      (event) => {
        const progress = event.payload

        if (progress.uploadId === item.upload.id) {
          setProgress(progress.progress)
          setRate(progress.rateBytes)
          setUploaded(progress.uploadedBytes)
        }
      }
    )

    const finishListener = listen<UploadRequest>(
      'upload-finish',
      (event) => {
        const request = event.payload

        if (request.upload.id === item.upload.id) {
          setProgress(request.progress)
          setRate(request.rateBytes)
          setUploaded(request.uploadedBytes)
        }
      }
    )

    return () => {
      progressListener.then(unlistenFn => unlistenFn())
      finishListener.then(unlistenFn => unlistenFn())
    }
  }, [item.upload.id])

  return (
    <>
      <Progress className="h-3" value={progress * 100}></Progress>
      <div className="flex place-content-between">
        <Small>{formatRate(rate)}</Small>
        <Small>{formatSizeBytes(uploaded)} / {formatSizeBytes(item.upload.size)}</Small>
      </div>
    </>
  )
}

export function UploadActions() {
  return (
    <>
      <Button variant="outline" size="icon">
        <XIcon />
      </Button>
    </>
  )
}