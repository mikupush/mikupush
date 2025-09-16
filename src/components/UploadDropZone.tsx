/**
 * Copyright 2025 Miku Push! Team
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
import { useTranslation } from 'react-i18next'

export default function UploadDropZone() {
  const { t } = useTranslation()

  return (
    <div className="w-full h-full flex absolute inset-0 p-[10px]">
      <div className="flex flex-1 items-center justify-center rounded-3xl text-4xl font-extrabold bg-primary/95 text-primary-foreground">
        {t('uploads.drop')}
      </div>
    </div>
  )
}