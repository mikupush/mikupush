import { useContext } from 'react'
import { ThemeProviderContext } from '@/context/ThemeProvider.tsx'
import { Theme } from '@/model/config.ts'
import { applyConfig, getConfig } from '@/helpers/config.ts'
import { CONFIG_THEME } from '@/constants/config.ts'

export function useUserTheme() {
  const { setTheme } = useContext(ThemeProviderContext)

  const apply = (theme: Theme) => {
    applyConfig(CONFIG_THEME, theme).then(() => setTheme(theme))
  }

  const current = () => {
    getConfig(CONFIG_THEME).then(theme => setTheme(theme as Theme))
  }

  return {
    apply,
    current
  }
}