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

import { Heading2 } from '@/components/Typography.tsx'
import { useTranslation } from 'react-i18next'
import { useParams } from 'react-router'
import { Switch } from '@/components/ui/switch.tsx'
import { Field, FieldContent, FieldError, FieldLabel } from '@/components/ui/field.tsx'
import { useEffect, useMemo, useState } from 'react'
import { Server } from '@/model/server.ts'
import { invoke } from '@tauri-apps/api/core'
import toast from 'react-hot-toast'
import LoadingSpinner from '@/components/LoadingSpinner.tsx'
import { Input } from '@/components/ui/input.tsx'
import { Button } from '@/components/ui/button.tsx'
import { Controller, useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import zod from 'zod'
import { CheckCircleIcon, LoaderCircle, TriangleAlertIcon } from 'lucide-react'
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert.tsx'

interface ServerEditFormValues {
  alias: string
  useAlias: boolean
  url: string
}

interface HealthCheckAlert {
  variant: 'success' | 'danger'
  title: string
  message: string
}

export default function ServerEditPage() {
  const { t } = useTranslation()
  const { id } = useParams<{ id: string }>()
  const [server, setServer] = useState<Server | null>(null)
  const [loading, setLoading] = useState(false)
  const [saving, setSaving] = useState(false)
  const [testingConnection, setTestingConnection] = useState(false)
  const [healthCheckAlert, setHealthCheckAlert] = useState<HealthCheckAlert | null>(null)
  const schema = useMemo(() => zod.object({
    alias: zod.string()
      .trim(),
    useAlias: zod.boolean(),
    url: zod.string()
      .trim()
      .min(1, t('server.form.url.error.required'))
      .url(t('server.form.url.error.format')),
  }), [t])
  const {
    control,
    formState: { errors },
    handleSubmit,
    register,
    reset,
    watch,
  } = useForm<ServerEditFormValues>({
    resolver: zodResolver(schema),
    defaultValues: {
      alias: '',
      useAlias: false,
      url: '',
    },
  })
  const useAlias = watch('useAlias')

  const buildServerFromForm = (values: ServerEditFormValues): Server | null => {
    if (!server) {
      return null
    }

    return {
      ...server,
      ...values,
      alias: values.useAlias && values.alias !== '' ? values.alias : null,
    }
  }

  const checkServerHealth = async (server: Server) => {
    setHealthCheckAlert(null)

    try {
      await invoke<void>('check_server_health', { server })
      setHealthCheckAlert({
        variant: 'success',
        title: t('server.form.health_check.success.title'),
        message: t('server.form.health_check.success.message'),
      })
    } catch (error) {
      const message = typeof error === 'string' ? error : t('errors.unknown')
      setHealthCheckAlert({
        variant: 'danger',
        title: t('server.form.health_check.error.title'),
        message,
      })
      throw error
    }
  }

  useEffect(() => {
    setLoading(true)
    invoke<Server | null>('get_server_by_id', { id })
      .then(result => {
        if (!result) {
          toast.error(t('errors.server.not_available'))
          return
        }

        setServer(result)
        reset({
          alias: result.alias ?? '',
          useAlias: result.useAlias,
          url: result.url,
        })
      })
      .catch(error => toast.error(error))
      .finally(() => setLoading(false))
  }, [id, reset, t])

  const onSubmit = async (values: ServerEditFormValues) => {
    const serverToSave = buildServerFromForm(values)
    if (!serverToSave) {
      return
    }

    const savePromise = checkServerHealth(serverToSave)
      .then(() => invoke<Server>('update_server', { server: {
        ...serverToSave,
        healthy: true,
      } }))
      .then(savedServer => {
        setServer(savedServer)
        reset({
          alias: savedServer.alias ?? '',
          useAlias: savedServer.useAlias,
          url: savedServer.url,
        })
      })

    setSaving(true)
    toast.promise(savePromise, {
      loading: t('server.form.save.loading'),
      success: t('server.form.save.success'),
      error: (error) => typeof error === 'string' ? error : t('errors.unknown'),
    })

    try {
      await savePromise
    } finally {
      setSaving(false)
    }
  }

  const onTestConnection = async (values: ServerEditFormValues) => {
    const serverToTest = buildServerFromForm(values)
    if (!serverToTest) {
      return
    }

    setTestingConnection(true)
    try {
      await checkServerHealth(serverToTest)
    } catch {
      // The alert state is set by checkServerHealth.
    } finally {
      setTestingConnection(false)
    }
  }

  if (loading) {
    return (
      <div className="flex flex-1 items-center justify-center">
        <LoadingSpinner />
      </div>
    )
  }

  if (!server) {
    return null
  }

  const controlsDisabled = saving || testingConnection

  return (
    <div className="p-5 max-w-lg">
      <Heading2 className="mb-6">{t('server.form.heading')}</Heading2>
      <form className="flex flex-1 flex-col gap-5" onSubmit={handleSubmit(onSubmit)}>
        <Field orientation="horizontal">
          <Controller
            control={control}
            name="useAlias"
            render={({ field }) => (
              <Switch
                id="switch-alias"
                checked={field.value}
                onCheckedChange={field.onChange}
                disabled={controlsDisabled}
              />
            )}
          />
          <FieldContent>
            <FieldLabel htmlFor="switch-alias">
              {t('server.form.use_alias')}
            </FieldLabel>
          </FieldContent>
        </Field>
        <Field>
          <FieldLabel>{t('server.form.alias.label')}</FieldLabel>
          <Input
            {...register('alias')}
            disabled={!useAlias || controlsDisabled}
          />
          <FieldError errors={[errors.alias]}/>
        </Field>
        <Field>
          <FieldLabel>{t('server.form.url.label')}</FieldLabel>
          <Input
            {...register('url')}
            aria-invalid={!!errors.url}
            disabled={controlsDisabled}
          />
          <FieldError errors={[errors.url]}/>
        </Field>
        <div className="flex gap-2">
          <Button
            type="button"
            variant="outline"
            disabled={controlsDisabled}
            onClick={handleSubmit(onTestConnection)}
          >
            {testingConnection && <LoaderCircle className="animate-spin" />}
            {t('server.form.test_connection')}
          </Button>
          <Button type="submit" disabled={controlsDisabled}>
            {saving && <LoaderCircle className="animate-spin" />}
            {t('common.form.save')}
          </Button>
        </div>
      </form>
      {healthCheckAlert && (
        <Alert className="mt-5" variant={healthCheckAlert.variant}>
          <div className="flex gap-3">
            {healthCheckAlert.variant === 'success' ? (
              <CheckCircleIcon className="mt-0.5 size-4 shrink-0" />
            ) : (
              <TriangleAlertIcon className="mt-0.5 size-4 shrink-0" />
            )}
            <div>
              <AlertTitle>{healthCheckAlert.title}</AlertTitle>
              <AlertDescription>{healthCheckAlert.message}</AlertDescription>
            </div>
          </div>
        </Alert>
      )}
    </div>
  )
}
