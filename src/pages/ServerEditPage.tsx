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

import { Large, Muted } from '@/components/Typography.tsx'
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
import { LoaderCircle } from 'lucide-react'
import { ServerIcon } from '@/components/ServerIcon.tsx'
import BackButton from '@/components/BackButton.tsx'
import { PageHeading } from '@/components/PageHeading.tsx'

interface ServerEditFormValues {
  alias: string
  useAlias: boolean
}

export default function ServerEditPage() {
  const { t } = useTranslation()
  const { id } = useParams<{ id: string }>()
  const [server, setServer] = useState<Server | null>(null)
  const [loading, setLoading] = useState(false)
  const [saving, setSaving] = useState(false)
  const schema = useMemo(() => zod.object({
    alias: zod.string()
      .trim(),
    useAlias: zod.boolean(),
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
    },
  })
  const useAlias = watch('useAlias')

  const buildServerFromForm = (values: ServerEditFormValues): Server | null => {
    if (!server) {
      return null
    }

    return {
      ...server,
      ...values
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

    const savePromise = invoke<Server>('update_server', { server: serverToSave })
      .then(savedServer => {
        setServer(savedServer)
        reset({
          alias: savedServer.alias ?? '',
          useAlias: savedServer.useAlias,
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

  const controlsDisabled = saving
  const name = (server.useAlias && server.alias != null && server.alias !== '')
    ? `${server.alias} (${server.name})`
    : server.name

  return (
    <div className="p-5 max-w-lg">
      <PageHeading
        backAction={<BackButton backTo="/servers" />}
        title={t('server.form.heading')}
      />
      <div className="flex items-center mb-6">
        <ServerIcon icon={server.icon} />
        <div className="ml-2">
          <Large className="mb-1">{name}</Large>
          <Muted>{server.url}</Muted>
        </div>
      </div>
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
        <div className="flex gap-2">
          <Button type="submit" disabled={saving}>
            {saving && <LoaderCircle className="animate-spin" />}
            {t('common.form.save')}
          </Button>
        </div>
      </form>
      <section className="mt-10 space-y-4">
        <Large className="text-red-500">{t('common.form.danger_zone')}</Large>
        <div className="flex flex-col items-start gap-4">
          <Button type="button" variant="destructive">
            {t('server.form.danger_zone.delete_server_and_uploads')}
          </Button>
          <Button
            type="button"
            className="bg-red-100 text-foreground hover:bg-red-200"
          >
            {t('server.form.danger_zone.delete_uploads')}
          </Button>
          <Button
            type="button"
            className="bg-red-100 text-foreground hover:bg-red-200"
          >
            {t('server.form.danger_zone.delete_server')}
          </Button>
        </div>
      </section>
    </div>
  )
}
