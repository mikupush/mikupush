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
import { Field, FieldError, FieldLabel } from '@/components/ui/field.tsx'
import { Select, SelectContent, SelectGroup, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select.tsx'
import { Language } from '@/model/config.ts'
import { useUserLanguage } from '@/hooks/use-configuration.ts'

export function LanguageField() {
  const { t } = useTranslation()
  const { applyLanguage, language } = useUserLanguage()

  return (
    <Field>
      <FieldLabel>{t('settings.general.language.label')}</FieldLabel>
      <Select
        value={language}
        onValueChange={(value) => applyLanguage(value as Language)}
      >
        <SelectTrigger className="w-full max-w-56">
          <SelectValue placeholder={t('settings.general.language.placeholder')}/>
        </SelectTrigger>
        <SelectContent>
          <SelectGroup>
            <SelectItem value="en">{t('settings.general.language.option.en')}</SelectItem>
            <SelectItem value="es">{t('settings.general.language.option.es')}</SelectItem>
          </SelectGroup>
        </SelectContent>
      </Select>
      <FieldError></FieldError>
    </Field>
  )
}
