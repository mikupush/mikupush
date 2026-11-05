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

import { useEffect, useRef, useState } from 'react'
import { useTranslation } from 'react-i18next'
import { useUploadsStore } from '@/store/uploads'
import { UploadPreferences } from '@/model/upload'
import { getUploadPreferences, saveUploadPreferences, startUploads } from '@/helpers/upload.ts'
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle } from '@/components/ui/dialog'
import { Field, FieldContent, FieldDescription, FieldGroup, FieldLabel, FieldLegend, FieldSet } from '@/components/ui/field'
import { Button } from '@/components/ui/button'
import { Switch } from '@/components/ui/switch'

export function UploadPreferencesButton() {
  const { t } = useTranslation()
  const open = useUploadsStore(state => state.queueUploadPreferences)
  return <Button variant="ghost" onClick={() => open()}>{t('uploads.preferences.title')}</Button>
}

export function UploadPreferencesDialog() {
  const session = useUploadsStore(state => state.preferenceDialogs[0])
  return session ? <PreferencesForm key={session.id} session={session} /> : null
}

function PreferencesForm({ session }: { session: { id: number, paths: string[] | null } }) {
  const { t } = useTranslation()
  const close = useUploadsStore(state => state.closeUploadPreferences)
  const [preferences, setPreferences] = useState<UploadPreferences | null>(null)
  const [busy, setBusy] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const submitting = useRef(false)
  const [returnFocus] = useState(() => document.activeElement)
  const automatic = session.paths !== null

  useEffect(() => {
    let active = true
    getUploadPreferences().then(value => {
      if (active) setPreferences(value)
    }).catch(() => {
      if (active) setError(t('uploads.preferences.load_error'))
    })
    return () => { active = false }
  }, [t])

  const cancel = () => {
    if (!submitting.current) close(session.id)
  }

  const confirm = async () => {
    if (!preferences || submitting.current) return
    submitting.current = true
    setBusy(true)
    setError(null)
    try {
      if (session.paths !== null) {
        await startUploads(session.paths, { ...preferences })
      } else {
        await saveUploadPreferences(preferences)
      }
      close(session.id)
    } catch {
      setError(t('uploads.preferences.save_error'))
    } finally {
      submitting.current = false
      setBusy(false)
    }
  }

  return (
    <Dialog open onOpenChange={open => { if (!open) cancel() }}>
      <DialogContent
        showCloseButton={!busy}
        className="max-h-[85vh] overflow-y-auto"
        aria-busy={busy || preferences === null}
        onCloseAutoFocus={event => {
          event.preventDefault()
          if (returnFocus instanceof HTMLElement && returnFocus.isConnected) returnFocus.focus()
        }}
      >
        <DialogHeader>
          <DialogTitle>{t('uploads.preferences.title')}</DialogTitle>
          <DialogDescription>
            {t(automatic ? 'uploads.preferences.automatic_description' : 'uploads.preferences.manual_description')}
          </DialogDescription>
        </DialogHeader>
        <FieldSet disabled={busy || preferences === null}>
          <FieldLegend>{t('uploads.preferences.security')}</FieldLegend>
          <FieldGroup>
            <Field orientation="horizontal">
              <Switch
                id="upload-encryption"
                aria-describedby="upload-encryption-description"
                checked={preferences?.encryption ?? false}
                disabled={busy || preferences === null}
                onCheckedChange={encryption => setPreferences(current => current && ({ ...current, encryption }))}
              />
              <FieldContent>
                <FieldLabel htmlFor="upload-encryption">{t('uploads.preferences.encryption')}</FieldLabel>
                <FieldDescription id="upload-encryption-description">
                  {t('uploads.preferences.encryption_description')}
                </FieldDescription>
              </FieldContent>
            </Field>
          </FieldGroup>
        </FieldSet>
        {/* Add password protection here; expiration and download limits can use separate FieldSets. */}
        {error && <p role="alert" className="text-destructive text-sm">{error}</p>}
        <DialogFooter>
          <Button variant="outline" disabled={busy} onClick={cancel}>{t('common.cancel')}</Button>
          <Button disabled={busy || preferences === null} onClick={confirm}>
            {t(automatic ? 'uploads.preferences.start' : 'uploads.preferences.save')}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
