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

import { Route, Routes } from 'react-router'
import DefaultLayout from '@/layout/DefaultLayout.tsx'
import UploadsPage from '@/pages/UploadsPage.tsx'
import SettingsPage from '@/pages/SettingsPage.tsx'
import ArchivedUploadsPage from '@/pages/ArchivedUploadsPage.tsx'

export default function Router() {
  return (
    <Routes>
      <Route element={<DefaultLayout />}>
        <Route index element={<UploadsPage />} />
        <Route path="settings" element={<SettingsPage />} />
        <Route path="archived" element={<ArchivedUploadsPage />} />
      </Route>
    </Routes>
  )
}