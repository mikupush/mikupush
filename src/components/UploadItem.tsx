import FileIcon from "@/components/FileIcon"
import { Large, Small } from "@/components/Typography"
import { Button } from "@/components/ui/button"
import { Progress } from "@/components/ui/progress"
import { extractExtension } from "@/helpers/file"
import { UploadRequest } from "@/model/upload"
import { XIcon } from "lucide-react"

interface UploadItemProps {
  item: UploadRequest
}

export default function UploadItem({ item }: UploadItemProps) {
  // TODO: usar signals para actualizar el progreso

  return (
    <div className="flex p-[10px]">
      <FileIcon extension={extractExtension(item.upload.name)} />
      <div className="flex flex-1 flex-col place-content-between mx-[10px]">
        <Large>{item.upload.name}</Large>
        <UploadProgress value={item.progress} />
      </div>
      <div className="flex items-center">
        <UploadActions />
      </div>
    </div>
  )
}

interface UploadProgressProps {
  value: number
}

export function UploadProgress({ value }: UploadProgressProps) {
  return (
    <>
      <Progress className="h-3" value={value}></Progress>
      <Small>40 MB/s</Small>
    </>
  )
}

interface UploadActionsProps {
  uploadId: string
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