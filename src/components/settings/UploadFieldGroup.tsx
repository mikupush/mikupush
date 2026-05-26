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
import { Field, FieldContent, FieldDescription, FieldGroup, FieldLabel } from '@/components/ui/field.tsx'
import { Switch } from '@/components/ui/switch.tsx'
import { Input } from '@/components/ui/input.tsx'
import { useConfigField } from '@/hooks/form.ts'
import {
  CONFIG_FALSE_VALUE,
  CONFIG_TRUE_VALUE,
  CONFIG_UPLOAD_CHUNK_SIZE,
  CONFIG_UPLOAD_IN_CHUNKS
} from '@/constants/config.ts'

export function UploadFieldGroup() {
  const { t } = useTranslation()
  const [chunkMode, setChunkMode] = useConfigField(CONFIG_UPLOAD_IN_CHUNKS, CONFIG_TRUE_VALUE)
  const [chunkSize, setChunkSize] = useConfigField(CONFIG_UPLOAD_CHUNK_SIZE, '50')

  return (
    <FieldGroup>
      <Field orientation="horizontal">
        <Switch
          id="switch-chunk-mode"
          checked={chunkMode === CONFIG_TRUE_VALUE}
          onCheckedChange={checked => {
            setChunkMode(checked ? CONFIG_TRUE_VALUE : CONFIG_FALSE_VALUE)
          }}
        />
        <FieldContent>
          <FieldLabel htmlFor="switch-chunk-mode">
            {t('settings.upload.upload_in_chunks.label')}
          </FieldLabel>
          <FieldDescription>
            {t('settings.upload.upload_in_chunks.description')}
          </FieldDescription>
        </FieldContent>
      </Field>
      <Field>
        <FieldLabel>
          {t('settings.upload.chunk_size.label')}
        </FieldLabel>
        <Input
          type="number"
          min="5"
          max="1000"
          defaultValue="50"
          disabled={chunkMode !== CONFIG_TRUE_VALUE}
          value={Number(chunkSize)}
          onChange={(event) => {
            setChunkSize(event.target.value)
          }}
        />
        <FieldDescription>
          {t('settings.upload.chunk_size.description')}
        </FieldDescription>
      </Field>
    </FieldGroup>
  )
}