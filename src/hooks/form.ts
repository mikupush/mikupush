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