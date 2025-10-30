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

import { useTheme } from '@/context/ThemeProvider.tsx'
import { Theme } from '@/model/config.ts'
import { applyConfig, getConfig } from '@/helpers/config.ts'
import { CONFIG_THEME } from '@/constants/config.ts'

export function useUserTheme() {
  const { setTheme, theme } = useTheme()

  const applyTheme = (theme: Theme) => {
    applyConfig(CONFIG_THEME, theme).then(() => setTheme(theme))
  }

  const currentTheme = () => {
    getConfig(CONFIG_THEME).then(theme => setTheme(theme as Theme))
  }

  return {
    applyTheme,
    currentTheme,
    theme
  }
}