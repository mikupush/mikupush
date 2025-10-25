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

import { ThemeProvider } from '@/context/ThemeProvider.tsx'
import appIcon from '@/assets/app-icon.svg'
import { getName, getVersion } from '@tauri-apps/api/app'
import { useEffect, useState } from 'react'
import { useTranslation } from 'react-i18next'
import { Large, Heading2, Paragraph } from '@/components/Typography.tsx'
import { Button } from '@/components/ui/button.tsx'

function AboutWindow() {
  const [appName, setAppName] = useState('')
  const [appVersion, setAppVersion] = useState('')
  const { t } = useTranslation()

  useEffect(() => {
    getName().then(name => setAppName(name))
    getVersion().then(version => setAppVersion(version))
  }, [])

  return (
    <ThemeProvider>
      <div className="py-6 px-20 space-y-6 overflow-auto">
        <div className="flex w-full place-content-center">
          <img src={appIcon} alt="logo" className="h-16" />
        </div>
        <div className="space-y-2">
          <Heading2 as="h1" className="text-center py-0 border-none">{appName}</Heading2>
          <Large className="text-center">{t('about.version', { version: appVersion })}</Large>
          <Large className="text-center">{t('about.copyright')}</Large>
          <Large className="text-center">{t('about.licensed')}</Large>
        </div>
        <div>
          <Paragraph className="text-center">{t('about.hatsune_miku_copyright')}</Paragraph>
          <Paragraph className="text-center">{t('about.not_affiliated')}</Paragraph>
          <Paragraph className="text-center">{t('about.why_hatsune_miku')}</Paragraph>
        </div>
        <div className="flex justify-center">
          <Button asChild><a href="#">{t('about.see_third_party_licenses')}</a></Button>
        </div>
      </div>
    </ThemeProvider>
  )
}

export default AboutWindow