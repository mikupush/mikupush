import FileIcon from "@/components/FileIcon"
import { Large, Small } from "@/components/Typography"
import { Button } from "@/components/ui/button"
import { Progress } from "@/components/ui/progress"
import { extractExtension } from "@/helpers/file"
import { UploadRequest } from "@/model/upload"
import { XIcon } from "lucide-react"
import {useEffect, useState} from "react"
import { listen } from "@tauri-apps/api/event"
import { ProgressEvent } from "@/model/events"
import { formatRate } from "@/helpers/rate"

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
  const [rate, setRate] = useState(formatRate(item.rateBytes))

  useEffect(() => {
    const progressListener = listen<ProgressEvent>(
      'upload-progress-changed',
      (event) => {
        const progressEvent = event.payload

        if (progressEvent.uploadId === item.upload.id) {
          setProgress(progressEvent.progress * 100)
          setRate(formatRate(progressEvent.rateBytes))
        }
      }
    )

    return () => {
      progressListener.then(unlistenFn => unlistenFn())
    }
  }, [item.upload.id])

  return (
    <>
      <Progress className="h-3" value={progress}></Progress>
      <Small>{rate}</Small>
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