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
      <FieldLabel>{t('settings.general.theme.label')}</FieldLabel>
      <Select
        value={theme}
        onValueChange={(value) => applyTheme(value as Theme)}
      >
        <SelectTrigger className="w-full max-w-56">
          <SelectValue placeholder={t('settings.general.theme.placeholder')}/>
        </SelectTrigger>
        <SelectContent>
          <SelectGroup>
            <SelectItem value="light">{t('settings.general.theme.option.light')}</SelectItem>
            <SelectItem value="dark">{t('settings.general.theme.option.dark')}</SelectItem>
            <SelectItem value="system">{t('settings.general.theme.option.system')}</SelectItem>
          </SelectGroup>
        </SelectContent>
      </Select>
      <FieldError></FieldError>
    </Field>
  )
}