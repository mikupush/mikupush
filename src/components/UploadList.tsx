import { UploadProgressItem, UploadItem } from "@/components/UploadItem";
import { UploadRequest } from "@/model/upload";

interface UploadListProps {
  items: UploadRequest[]
}

export default function UploadList({ items }: UploadListProps) {
  return (
    <div className="overflow-y-scroll flex-1">
      {items.map(item => <UploadItem key={item.upload.id} item={item} />)}
    </div>
  )
}

export function UploadProgressList({ items }: UploadListProps) {
  return (
    <div className="overflow-y-scroll flex-1">
      {items.map(item => <UploadProgressItem key={item.upload.id} item={item} />)}
    </div>
  )
}
