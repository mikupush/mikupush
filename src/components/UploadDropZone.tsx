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

export default function UploadDropZone() {
  const { t } = useTranslation()

  return (
    <div className="w-full h-full flex absolute inset-0 p-[10px]">
      <div className="flex flex-1 items-center justify-center rounded-3xl text-4xl font-extrabold bg-primary/95 text-primary-foreground">
        {t('uploads.drop')}
      </div>
    </div>
  )
}