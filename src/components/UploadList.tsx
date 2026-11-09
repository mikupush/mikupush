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

import { ArchivedUploadItem, UploadProgressItem, UploadItem } from '@/components/UploadItem'
import { Upload, UploadRequest } from '@/model/upload'
import { ReactNode } from 'react'
import { useTranslation } from 'react-i18next'

interface UploadListProps {
  items: UploadRequest[]
}

function UploadTable({ children }: { children: ReactNode }) {
  const { t } = useTranslation()

  return (
    <div className="min-w-0 flex-1 px-5">
      <table className="w-full table-fixed text-sm">
        <caption className="sr-only">{t('uploads.table.caption')}</caption>
        <colgroup>
          <col />
          <col className="w-24" />
          <col className="w-28" />
          <col className="w-40" />
          <col className="w-24" />
          <col className="w-36" />
          <col className="w-20" />
        </colgroup>
        <thead className="sticky top-0 z-10 border-b bg-background text-left text-muted-foreground">
          <tr>
            <th scope="col" className="px-3 py-4 font-medium">{t('uploads.table.name')}</th>
            <th scope="col" className="px-3 py-4 text-right font-medium">{t('uploads.table.size')}</th>
            <th scope="col" className="px-3 py-4 text-right font-medium">{t('uploads.table.speed')}</th>
            <th scope="col" className="px-3 py-4 font-medium">{t('uploads.table.status')}</th>
            <th scope="col" className="px-3 py-4 text-right font-medium">{t('uploads.table.progress')}</th>
            <th scope="col" className="px-3 py-4 font-medium">{t('uploads.table.created_at')}</th>
            <th scope="col" className="px-3 py-4"><span className="sr-only">{t('uploads.table.actions')}</span></th>
          </tr>
        </thead>
        <tbody className="divide-y">{children}</tbody>
      </table>
    </div>
  )
}

export default function UploadList({ items }: UploadListProps) {
  return <UploadTable>{items.map(item => <UploadItem key={item.upload.id} item={item} />)}</UploadTable>
}

export function UploadProgressList({ items }: UploadListProps) {
  return <UploadTable>{items.map(item => <UploadProgressItem key={item.upload.id} item={item} />)}</UploadTable>
}

interface ArchivedUploadListProps {
  items: Upload[]
}

export function ArchivedUploadList({ items }: ArchivedUploadListProps) {
  return <UploadTable>{items.map(item => <ArchivedUploadItem key={item.id} upload={item} />)}</UploadTable>
}
