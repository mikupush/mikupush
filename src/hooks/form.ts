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

import { ConfigKey } from '@/constants/config.ts'
import { useEffect, useState } from 'react'
import { applyConfig, getConfig } from '@/helpers/config.ts'
import toast from 'react-hot-toast'
import { useTranslation } from 'react-i18next'

export function useConfigField(key: ConfigKey, defaultValue = ''): [string, (value: string) => void] {
  const { t } = useTranslation()
  const [value, setValue] = useState<string>(defaultValue)

  useEffect(() => {
    getConfig(key).then(value => setValue(value))
  }, [key])

  const updateValue = (value: string) => {
    applyConfig(key, value).catch((error) => {
      console.error('error applying config', error)
      toast.error(t('errors.unknown'))
    })

    setValue(value)
  }

  return [value, updateValue]
}