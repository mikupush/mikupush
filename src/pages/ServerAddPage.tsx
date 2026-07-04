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

import BackButton from '@/components/BackButton.tsx'
import { PageHeading } from '@/components/PageHeading.tsx'
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert.tsx'
import { Button } from '@/components/ui/button.tsx'
import { Field, FieldContent, FieldError, FieldLabel } from '@/components/ui/field.tsx'
import { Input } from '@/components/ui/input.tsx'
import { Switch } from '@/components/ui/switch.tsx'
import { useServerConnector } from '@/hooks/server.ts'
import { createServerFromUrl, CreateServer, Server } from '@/model/server.ts'
import { zodResolver } from '@hookform/resolvers/zod'
import { invoke } from '@tauri-apps/api/core'
import { CheckCircleIcon, LoaderCircle, TriangleAlertIcon } from 'lucide-react'
import { useMemo, useState } from 'react'
import { Controller, useForm } from 'react-hook-form'
import toast from 'react-hot-toast'
import { useTranslation } from 'react-i18next'
import { useNavigate } from 'react-router'
import zod from 'zod'

interface ServerAddFormValues {
  alias: string
  useAlias: boolean
  url: string
}

interface ConnectionAlert {
  variant: 'success' | 'danger'
  title: string
  message: string
}

interface ServerInfo {
  name: string
  version: {
    name: string
    code: number
  }
}

export default function ServerAddPage() {
  const { t } = useTranslation()
  const navigate = useNavigate()
  const { connectById, isConnecting } = useServerConnector()
  const [saving, setSaving] = useState(false)
  const [savingAndConnecting, setSavingAndConnecting] = useState(false)
  const [testingConnection, setTestingConnection] = useState(false)
  const [connectionAlert, setConnectionAlert] = useState<ConnectionAlert | null>(null)
  const schema = useMemo(() => zod.object({
    alias: zod.string()
      .trim()
      .max(255, t('server.form.alias.error.max')),
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
    watch,
  } = useForm<ServerAddFormValues>({
    resolver: zodResolver(schema),
    defaultValues: {
      alias: '',
      useAlias: false,
      url: '',
    },
  })
  const useAlias = watch('useAlias')

  const buildCreateServerFromForm = (values: ServerAddFormValues): CreateServer => {
    const alias = values.alias.trim()
    const useAlias = values.useAlias && alias !== ''

    return {
      alias: useAlias ? alias : null,
      useAlias,
      url: values.url.trim(),
    }
  }

  const buildServerFromForm = (values: ServerAddFormValues): Server => {
    const createServer = buildCreateServerFromForm(values)

    return {
      ...createServerFromUrl(createServer.url),
      alias: createServer.alias,
      useAlias: createServer.useAlias,
    }
  }

  const testServerConnection = async (server: Server) => {
    setConnectionAlert(null)

    try {
      await invoke<void>('check_server_health', { server })
      const serverInfo = await invoke<ServerInfo>('fetch_server_info', { server })
      setConnectionAlert({
        variant: 'success',
        title: t('server.form.health_check.success.title'),
        message: t('server.form.health_check.success.message', {
          name: serverInfo.name,
          url: server.url,
          version: serverInfo.version.name,
        }),
      })
    } catch (error) {
      const message = typeof error === 'string' ? error : t('errors.unknown')
      setConnectionAlert({
        variant: 'danger',
        title: t('server.form.health_check.error.title'),
        message,
      })
      throw error
    }
  }

  const checkServerHealth = async (server: Server) => {
    await invoke<void>('check_server_health', { server })
  }

  const saveServer = async (server: CreateServer) => {
    return await invoke<Server>('create_server', { newServer: server })
  }

  const onSave = async (values: ServerAddFormValues) => {
    const savePromise = saveServer(buildCreateServerFromForm(values))
      .then(() => navigate('/servers'))

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

  const onSaveAndConnect = async (values: ServerAddFormValues) => {
    const serverToSave = buildServerFromForm(values)
    const saveAndConnectPromise = checkServerHealth(serverToSave)
      .then(() => saveServer(buildCreateServerFromForm(values)))
      .then(savedServer => connectById(savedServer.id))
      .then(() => navigate('/servers'))

    setSavingAndConnecting(true)
    toast.promise(saveAndConnectPromise, {
      loading: t('server.form.save_and_connect.loading'),
      success: t('server.form.save_and_connect.success'),
      error: (error) => typeof error === 'string' ? error : t('errors.unknown'),
    })

    try {
      await saveAndConnectPromise
    } finally {
      setSavingAndConnecting(false)
    }
  }

  const onTestConnection = async (values: ServerAddFormValues) => {
    const serverToTest = buildServerFromForm(values)

    setTestingConnection(true)

    try {
      await testServerConnection(serverToTest)
    } finally {
      setTestingConnection(false)
    }
  }

  const controlsDisabled = saving || savingAndConnecting || testingConnection || isConnecting

  return (
    <div className="p-5 max-w-lg">
      <PageHeading
        backAction={<BackButton backTo="/servers" />}
        title={t('server.form.add_heading')}
      />
      <form className="flex flex-1 flex-col gap-5" onSubmit={handleSubmit(onSave)}>
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
            aria-invalid={!!errors.alias}
            disabled={!useAlias || controlsDisabled}
            maxLength={255}
            placeholder={t('server.form.alias.placeholder')}
          />
          <FieldError errors={[errors.alias]}/>
        </Field>
        <Field>
          <FieldLabel>{t('server.form.url.label')}</FieldLabel>
          <Input
            {...register('url')}
            aria-invalid={!!errors.url}
            disabled={controlsDisabled}
            placeholder="https://mikupush.io"
          />
          <FieldError errors={[errors.url]}/>
        </Field>
        <div className="flex flex-wrap gap-2">
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
          <Button
            type="button"
            disabled={controlsDisabled}
            onClick={handleSubmit(onSaveAndConnect)}
          >
            {savingAndConnecting && <LoaderCircle className="animate-spin" />}
            {t('server.form.save_and_connect.label')}
          </Button>
        </div>
      </form>
      {connectionAlert && (
        <Alert className="mt-5" variant={connectionAlert.variant}>
          <div className="flex gap-3">
            {connectionAlert.variant === 'success' ? (
              <CheckCircleIcon className="mt-0.5 size-4 shrink-0" />
            ) : (
              <TriangleAlertIcon className="mt-0.5 size-4 shrink-0" />
            )}
            <div>
              <AlertTitle>{connectionAlert.title}</AlertTitle>
              <AlertDescription className="whitespace-pre-line">{connectionAlert.message}</AlertDescription>
            </div>
          </div>
        </Alert>
      )}
    </div>
  )
}
