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
      <FieldLabel>{t('settings.appearance.theme.label')}</FieldLabel>
      <Select
        value={theme}
        onValueChange={(value) => applyTheme(value as Theme)}
      >
        <SelectTrigger className="w-full max-w-56">
          <SelectValue placeholder={t('settings.appearance.theme.placeholder')}/>
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