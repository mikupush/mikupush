import { Route, Routes } from 'react-router'
import DefaultLayout from '@/layout/DefaultLayout.tsx'
import UploadsPage from '@/pages/UploadsPage.tsx'

export default function Router() {
  return (
    <Routes>
      <Route element={<DefaultLayout />}>
        <Route index element={<UploadsPage />} />
      </Route>
    </Routes>
  )
}