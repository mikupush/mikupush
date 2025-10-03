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