import { useTranslation } from 'react-i18next'
import { useServer } from '@/context/ServerProvider.tsx'
import { useState } from 'react'
import toast from 'react-hot-toast'
import { Field, FieldDescription, FieldError, FieldLabel } from '@/components/ui/field.tsx'
import { Input } from '@/components/ui/input.tsx'
import { Button } from '@/components/ui/button.tsx'
import zod from 'zod'

export function ServerField() {
  const { t } = useTranslation()
  const { setCurrentByUrl, current } = useServer()
  const [url, setUrl] = useState(current.url)
  const [errors, setErrors] = useState<string[]>([])

  const handleValidationError = (error: unknown): boolean => {
    if (!(error instanceof zod.ZodError)) {
      return false
    }

    setErrors(
      error.issues.map(issue => {
        if (issue.code === 'invalid_type') return t('settings.server.error.invalid')
        if (issue.code === 'invalid_format') return t('settings.server.error.format')
        if (issue.code === 'too_small') return t('settings.server.error.required')
        if (issue.input === undefined) return t('settings.server.error.required')
        return ''
      }).filter(error => error !== '')
    )
    return true
  }

  const handleServerError = (error: unknown) => {
    if (typeof error === 'string') {
      setErrors([error])
      return
    }

    if (error instanceof Error && error.message) {
      setErrors([error.message])
      return
    }

    toast.error(t('errors.unknown'))
  }

  const handleChangeServer = async () => {
    try {
      setErrors([])
      zod.url().nonempty().parse(url)
      await setCurrentByUrl(url)
      toast.success(t('settings.server.success'))
    } catch (error) {
      if (handleValidationError(error)) return
      handleServerError(error)
    }
  }

  return (
    <Field>
      <FieldLabel>{t('settings.server.label')}</FieldLabel>
      <div className="flex w-full max-w-lg items-center gap-2">
        <Input
          name="serverUrl"
          placeholder="https://mikupush.io"
          value={url}
          onChange={(event) => setUrl(event.target.value.toLowerCase().trim())}
        />
        <Button onClick={handleChangeServer}>{t('settings.server.apply')}</Button>
      </div>
      <FieldDescription>{t('settings.server.description')}</FieldDescription>
      <FieldError errors={errors.map(error => ({ message: error }))}/>
    </Field>
  )
}