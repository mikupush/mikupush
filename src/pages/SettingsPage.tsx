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
import { Heading2 } from '@/components/Typography.tsx'
import { FieldGroup, FieldLegend, FieldSet } from '@/components/ui/field.tsx'
import { ServerField } from '@/components/settings/ServerField.tsx'
import { ThemeField } from '@/components/settings/ThemeField.tsx'
import { UploadFieldGroup } from '@/components/settings/UploadFieldGroup.tsx'

export default function SettingsPage() {
  const { t } = useTranslation()

  return (
    <div className="p-5 max-w-lg">
      <Heading2 className="mb-6">{t('settings.heading')}</Heading2>
      <div className="space-y-6">
        <FieldSet className="space-y-6">
          <FieldLegend>{t('settings.appearance.heading')}</FieldLegend>
          <FieldGroup>
            <ThemeField />
          </FieldGroup>
        </FieldSet>
        <FieldSet className="space-y-6">
          <FieldLegend>{t('settings.upload.heading')}</FieldLegend>
          <UploadFieldGroup />
        </FieldSet>
        <FieldSet>
          <FieldLegend className="text-red-500">{t('common.form.danger_zone')}</FieldLegend>
          <FieldGroup>
            <ServerField />
          </FieldGroup>
        </FieldSet>
      </div>
    </div>
  )
}

