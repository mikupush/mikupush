import FileIcon from "@/components/FileIcon"
import { Large, Small } from "@/components/Typography"
import { Button } from "@/components/ui/button"
import { Progress } from "@/components/ui/progress"
import { FileDetails } from "@/model/file-details"
import { XIcon } from "lucide-react"
//import { UploadProgress } from "@/model/upload-progress"

interface UploadItemProps {
  //progress: UploadProgress
  file: FileDetails
}

export default function UploadItem() {
  return (
    <div className="flex p-[10px]">
      <FileIcon extension="png" />
      <div className="flex flex-1 flex-col place-content-between mx-[10px]">
        <Large>oranges.png</Large>
        <UploadProgress />
      </div>
      <div className="flex items-center">
        <UploadActions />
      </div>
    </div>
  )
}

export function UploadProgress() {
  return (
    <>
      <Progress className="h-3" value={50}></Progress>
      <Small>40 MB/s</Small>
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