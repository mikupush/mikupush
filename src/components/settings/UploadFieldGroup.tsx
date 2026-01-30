import { useTranslation } from 'react-i18next'
import { Field, FieldContent, FieldDescription, FieldGroup, FieldLabel } from '@/components/ui/field.tsx'
import { Switch } from '@/components/ui/switch.tsx'
import { Input } from '@/components/ui/input.tsx'
import { useConfigField } from '@/hooks/form.ts'
import {
  CONFIG_FALSE_VALUE,
  CONFIG_TRUE_VALUE,
  CONFIG_UPLOAD_CHUNK_SIZE,
  CONFIG_UPLOAD_IN_CHUNKS
} from '@/constants/config.ts'

export function UploadFieldGroup() {
  const { t } = useTranslation()
  const [chunkMode, setChunkMode] = useConfigField(CONFIG_UPLOAD_IN_CHUNKS, CONFIG_TRUE_VALUE)
  const [chunkSize, setChunkSize] = useConfigField(CONFIG_UPLOAD_CHUNK_SIZE, '50')

  return (
    <FieldGroup>
      <Field orientation="horizontal">
        <Switch
          id="switch-chunk-mode"
          checked={chunkMode === CONFIG_TRUE_VALUE}
          onCheckedChange={checked => {
            setChunkMode(checked ? CONFIG_TRUE_VALUE : CONFIG_FALSE_VALUE)
          }}
        />
        <FieldContent>
          <FieldLabel htmlFor="switch-chunk-mode">
            {t('settings.upload.upload_in_chunks.label')}
          </FieldLabel>
          <FieldDescription>
            {t('settings.upload.upload_in_chunks.description')}
          </FieldDescription>
        </FieldContent>
      </Field>
      <Field>
        <FieldLabel>
          {t('settings.upload.chunk_size.label')}
        </FieldLabel>
        <Input
          type="number"
          min="5"
          max="1000"
          defaultValue="50"
          disabled={chunkMode !== CONFIG_TRUE_VALUE}
          value={Number(chunkSize)}
          onChange={(event) => {
            setChunkSize(event.target.value)
          }}
        />
        <FieldDescription>
          {t('settings.upload.chunk_size.description')}
        </FieldDescription>
      </Field>
    </FieldGroup>
  )
}