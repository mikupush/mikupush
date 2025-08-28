import { useTranslation } from 'react-i18next'
import { Paragraph } from '@/components/Typography'
import { Button } from './ui/button'
import { FolderIcon } from 'lucide-react'
import { selectFiles } from '@/helpers/file'
import { UploadProgressList } from '@/components/UploadList'
import { useUploadsStore } from '@/store/uploads'
import UploadDropZone from '@/components/UploadDropZone'

export default function Uploads() {
  const inProgressUploads = useUploadsStore(state => state.inProgressUploads)
  const activeDropZone = useUploadsStore(state => state.activeDropZone)

  return (
    <div className="relative flex flex-1">
      {(inProgressUploads.length > 0) ? (
        <UploadProgressList items={inProgressUploads} />
      ) : (
        <EmptyState />
      )}

      {(activeDropZone) && (
        <UploadDropZone />
      )}
    </div>
  )
}

function EmptyState() {
  const { t } = useTranslation()

  return (
    <div className="flex flex-1 flex-col justify-center items-center px-20">
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