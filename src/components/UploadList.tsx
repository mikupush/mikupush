import { UploadProgressItem, UploadItem } from "@/components/UploadItem";
import { UploadRequest } from "@/model/upload";

interface UploadListProps {
  items: UploadRequest[]
}

export default function UploadList({ items }: UploadListProps) {
  return (
    <div className="overflow-y-scroll">
      {items.map((item, index) => <UploadItem key={index} item={item} />)}
    </div>
  )
}

export function UploadProgressList({ items }: UploadListProps) {
  return (
    <div className="overflow-y-scroll">
      {items.map((item, index) => <UploadProgressItem key={index} item={item} />)}
    </div>
  )
}
