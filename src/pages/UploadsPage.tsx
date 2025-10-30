/**
 * Miku Push! is a simple, lightweight, and open-source WeTransfer alternative for desktop.
 * Copyright (C) 2025  Miku Push! Team
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 * 
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

import { useTranslation } from 'react-i18next'
import { Paragraph } from '@/components/Typography.tsx'
import { Button } from '../components/ui/button.tsx'
import { FolderIcon } from 'lucide-react'
import { selectFiles } from '@/helpers/file.ts'
import { UploadProgressList } from '@/components/UploadList.tsx'
import { useUploadsStore } from '@/store/uploads.ts'
import UploadDropZone from '@/components/UploadDropZone.tsx'
import LoadingSpinner from '@/components/LoadingSpinner.tsx'

export default function UploadsPage() {
  const inProgressUploads = useUploadsStore(state => state.inProgressUploads)
  const activeDropZone = useUploadsStore(state => state.activeDropZone)
  const isLoading = useUploadsStore(state => state.isLoading)

  return (
    <div className="flex flex-1">
      {(isLoading) ? (
        <LoadingSpinner size={50} />
      ) : (inProgressUploads.length > 0) ? (
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
      <Paragraph className="text-center">{t('uploads.empty_state')}</Paragraph>
      <Button
        className="mt-5"
        onClick={() => selectFiles()}
      >
        {t('uploads.select_file')}
      </Button>
    </div>
  )
}