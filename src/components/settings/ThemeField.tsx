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
import { useUserTheme } from '@/hooks/use-configuration.ts'
import { Field, FieldError, FieldLabel } from '@/components/ui/field.tsx'
import { Select, SelectContent, SelectGroup, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select.tsx'
import { Theme } from '@/model/config.ts'

export function ThemeField() {
  const { t } = useTranslation()
  const { applyTheme, theme } = useUserTheme()

  return (
    <Field>
      <FieldLabel>{t('settings.general.theme.label')}</FieldLabel>
      <Select
        value={theme}
        onValueChange={(value) => applyTheme(value as Theme)}
      >
        <SelectTrigger className="w-full max-w-56">
          <SelectValue placeholder={t('settings.general.theme.placeholder')}/>
        </SelectTrigger>
        <SelectContent>
          <SelectGroup>
            <SelectItem value="light">{t('settings.general.theme.option.light')}</SelectItem>
            <SelectItem value="dark">{t('settings.general.theme.option.dark')}</SelectItem>
            <SelectItem value="system">{t('settings.general.theme.option.system')}</SelectItem>
          </SelectGroup>
        </SelectContent>
      </Select>
      <FieldError></FieldError>
    </Field>
  )
}