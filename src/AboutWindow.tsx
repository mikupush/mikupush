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

import { ThemeProvider } from '@/context/ThemeProvider.tsx'
import appIcon from '@/assets/app-icon.svg'
import { getName, getVersion } from '@tauri-apps/api/app'
import { useEffect, useState } from 'react'
import { useTranslation } from 'react-i18next'
import { Large, Heading2, Paragraph } from '@/components/Typography.tsx'
import { Button } from '@/components/ui/button.tsx'
import { resourcePath } from '@/helpers/resource.ts'
import { openUrl } from '@tauri-apps/plugin-opener'
import toast from 'react-hot-toast'
import { ToastContainer } from '@/components/ToastContainer.tsx'

function AboutWindow() {
  const [appName, setAppName] = useState('')
  const [appVersion, setAppVersion] = useState('')
  const { t } = useTranslation()

  useEffect(() => {
    getName().then(name => setAppName(name))
    getVersion().then(version => setAppVersion(version))
  }, [])

  const openThirdPartyLicenses = async () => {
    try {
      const path = await resourcePath('third_party_licenses')
      const url = encodeURI(`file://${path}`)
      await openUrl(url)
    } catch (error) {
      console.error('Error opening third-party licenses file:', error)
      toast.error(t('errors.about.third_party_licenses_open'))
    }
  }

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
          <Button onClick={openThirdPartyLicenses}>{t('about.see_third_party_licenses')}</Button>
        </div>
      </div>
      <ToastContainer />
    </ThemeProvider>
  )
}

export default AboutWindow