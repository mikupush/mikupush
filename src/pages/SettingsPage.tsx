import { Input } from '@/components/ui/input.tsx'
import { Form, FormControl, FormDescription, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form.tsx'
import zod from 'zod'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { useTranslation } from 'react-i18next'
import { Button } from '@/components/ui/button.tsx'
import { Heading2, Heading3 } from '@/components/Typography.tsx'
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

export default function SettingsPage() {
  const { t } = useTranslation()
  const { apply } = useUserTheme()

  const theme = zod.enum(
    ['light', 'dark', 'system'],
    {
      error: issue => {
        if (issue.input === undefined) return t('settings.appearance.theme.error.required')
        if (issue.code === 'invalid_value') return t('settings.appearance.theme.error.invalid')
        return undefined
      }
    }
  )

  const schema = zod.object({
    theme: theme,
    serverUrl: zod.url({
      error: issue => {
        if (issue.input === undefined) return t('settings.server.error.required')
        if (issue.code === 'invalid_type') return t('settings.server.error.invalid')
        if (issue.code === 'invalid_format') return t('settings.server.error.format')
        return undefined
      }
    }).nonempty({
      error: issue => {
        if (issue.code === 'too_small') return t('settings.server.error.required')
        return undefined
      }
    }),
  })

  const form = useForm({
    resolver: zodResolver(schema),
    defaultValues: {
      theme: 'system' as Theme,
      serverUrl: 'https://mikupush.io',
    },
  })

  const saveSettings = (data: zod.infer<typeof schema>) => {
    console.log(data)
    apply(data.theme)
  }

  return (
    <div className="p-5 max-w-lg">
      <Heading2>{t('settings.heading')}</Heading2>
      <Form {...form}>
        <form className="py-5 space-y-6" onSubmit={form.handleSubmit(saveSettings)}>
          <fieldset className="space-y-6">
            <Heading3 as="legend">{t('settings.appearance.heading')}</Heading3>
            <FormField
              name="theme"
              control={form.control}
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t('settings.appearance.theme.label')}</FormLabel>
                  <Select
                    value={field.value}
                    onValueChange={field.onChange}
                  >
                    <FormControl>
                      <SelectTrigger className="w-full sm:w-56">
                        <SelectValue placeholder={t('settings.appearance.theme.placeholder')} />
                      </SelectTrigger>
                    </FormControl>
                    <SelectContent>
                      <SelectGroup>
                        <SelectItem value="light">{t('settings.appearance.theme.option.light')}</SelectItem>
                        <SelectItem value="dark">{t('settings.appearance.theme.option.dark')}</SelectItem>
                        <SelectItem value="system">{t('settings.appearance.theme.option.system')}</SelectItem>
                      </SelectGroup>
                    </SelectContent>
                  </Select>
                  <FormMessage />
                </FormItem>
              )}
            />
          </fieldset>
          <fieldset className="space-y-6">
            <Heading3 as="legend" className="text-red-500">{t('common.form.danger_zone')}</Heading3>
            <FormField
              name="serverUrl"
              control={form.control}
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t('settings.server.label')}</FormLabel>
                  <FormControl>
                    <Input placeholder="https://mikupush.io" {...field} />
                  </FormControl>
                  <FormDescription>{t('settings.server.description')}</FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />
          </fieldset>
          <Button type="submit">{t('common.form.save')}</Button>
        </form>
      </Form>
    </div>
  )
}