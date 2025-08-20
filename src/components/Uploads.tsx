import { useTranslation } from "react-i18next";
import { Paragraph } from '@/components/Typography'
import { Button } from "./ui/button";
import { FolderIcon } from "lucide-react";
import { selectFiles } from "@/helpers/file";
import { UploadProgressList } from "@/components/UploadList";
import { useUploadsStore } from "@/store/uploads";

export default function Uploads() {
  const inProgressUploads = useUploadsStore(state => state.inProgressUploads)

  return (
    (inProgressUploads.length > 0) ? (
      <UploadProgressList items={inProgressUploads} />
    ) : (
      <EmptyState />
    )
  )
}

function EmptyState() {
  const { t } = useTranslation();

  return (
    <div className="flex flex-col justify-center items-center m-auto w-2/3">
      <FolderIcon width={60} height={60} />
      <Paragraph align="center">{t('uploads.empty_state')}</Paragraph>
      <Button 
        className="mt-5" 
        onClick={() => selectFiles()}
      >
        {t('uploads.select_file')}
      </Button>
    </div>
  )
}