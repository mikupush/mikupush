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