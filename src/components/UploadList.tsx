import UploadItem from "@/components/UploadItem";
import { UploadRequest } from "@/model/upload";

interface UploadListProps {
  items: UploadRequest[]
}

export default function UploadList({ items }: UploadListProps) {
  return (
    <>
      {items.map(item => <UploadItem item={item} />)}
    </>
  )
}
