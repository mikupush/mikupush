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

import { Field, FieldContent, FieldLabel } from '@/components/ui/field.tsx'
import { useTranslation } from 'react-i18next'
import { Switch } from '@/components/ui/switch.tsx'
import { enable, disable, isEnabled } from '@tauri-apps/plugin-autostart'
import { useEffect, useState } from 'react'

export function AutoStartField() {
  const { t } = useTranslation()
  const [enabled, setEnabled] = useState(false)

  useEffect(() => {
    isEnabled().then(setEnabled)
  }, [])

  const toggleAutoStart = async () => {
    const isCurrentlyEnabled = await isEnabled()
    if (!isCurrentlyEnabled) {
      await enable()
    } else {
      await disable()
    }

    setEnabled(!isCurrentlyEnabled)
  }

  return (
    <Field orientation="horizontal">
      <Switch
        id="switch-autostart"
        checked={enabled}
        onCheckedChange={() => toggleAutoStart()}
      />
      <FieldContent>
        <FieldLabel htmlFor="switch-autostart">
          {t('settings.general.autostart.label')}
        </FieldLabel>
      </FieldContent>
    </Field>
  )
}