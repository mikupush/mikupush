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

import { Input } from '@/components/ui/input.tsx'
import { useTranslation } from 'react-i18next'
import { Button } from '@/components/ui/button.tsx'
import { Heading2 } from '@/components/Typography.tsx'
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue
} from '@/components/ui/select.tsx'
import { Theme } from '@/model/config.ts'
import { useUserTheme } from '@/hooks/use-configuration.ts'
import {
  Field,
  FieldDescription,
  FieldError,
  FieldGroup,
  FieldLabel,
  FieldLegend,
  FieldSet
} from '@/components/ui/field.tsx'
import { useServer } from '@/context/ServerProvider.tsx'
import { useState } from 'react'
import zod from 'zod'
import toast from 'react-hot-toast'

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

function ThemeField() {
  const { t } = useTranslation()
  const { applyTheme, theme } = useUserTheme()

  return (
    <Field>
      <FieldLabel>{t('settings.appearance.theme.label')}</FieldLabel>
      <Select
        value={theme}
        onValueChange={(value) => applyTheme(value as Theme)}
      >
        <SelectTrigger className="w-full max-w-56">
          <SelectValue placeholder={t('settings.appearance.theme.placeholder')} />
        </SelectTrigger>
        <SelectContent>
          <SelectGroup>
            <SelectItem value="light">{t('settings.appearance.theme.option.light')}</SelectItem>
            <SelectItem value="dark">{t('settings.appearance.theme.option.dark')}</SelectItem>
            <SelectItem value="system">{t('settings.appearance.theme.option.system')}</SelectItem>
          </SelectGroup>
        </SelectContent>
      </Select>
      <FieldError></FieldError>
    </Field>
  )
}

function ServerField() {
  const { t } = useTranslation()
  const { setCurrentByUrl, current } = useServer()
  const [url, setUrl] = useState(current.url)
  const [errors, setErrors] = useState<string[]>([])

  const handleValidationError = (error: unknown): boolean => {
    if (!(error instanceof zod.ZodError)) {
      return false
    }

    setErrors(
      error.issues.map(issue => {
        if (issue.code === 'invalid_type') return t('settings.server.error.invalid')
        if (issue.code === 'invalid_format') return t('settings.server.error.format')
        if (issue.code === 'too_small') return t('settings.server.error.required')
        if (issue.input === undefined) return t('settings.server.error.required')
        return ''
      }).filter(error => error !== '')
    )
    return true
  }

  const handleServerError = (error: unknown) => {
    if (typeof error === 'string') {
      setErrors([error])
      return
    }

    if (error instanceof Error && error.message) {
      setErrors([error.message])
      return
    }

    toast.error(t('errors.unknown'))
  }

  const handleChangeServer = async () => {
    try {
      setErrors([])
      zod.url().nonempty().parse(url)
      await setCurrentByUrl(url)
      toast.success(t('settings.server.success'))
    } catch (error) {
      if (handleValidationError(error)) return
      handleServerError(error)
    }
  }

  return (
    <Field>
      <FieldLabel>{t('settings.server.label')}</FieldLabel>
      <div className="flex w-full max-w-lg items-center gap-2">
        <Input
          name="serverUrl"
          placeholder="https://mikupush.io"
          value={url}
          onChange={(event) => setUrl(event.target.value.toLowerCase().trim())}
        />
        <Button onClick={handleChangeServer}>{t('settings.server.apply')}</Button>
      </div>
      <FieldDescription>{t('settings.server.description')}</FieldDescription>
      <FieldError errors={errors.map(error => ({ message: error }))} />
    </Field>
  )
}