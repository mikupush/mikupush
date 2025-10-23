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

import { Toaster } from 'react-hot-toast'
import { ThemeProvider } from '@/context/ThemeProvider.tsx'
import Router from '@/router.tsx'
import { ServerProvider } from '@/context/ServerProvider.tsx'

function AboutWindow() {
  return (
    <ThemeProvider>
      <ServerProvider>
        <Router />
        <Toaster
          position="bottom-right"
          toastOptions={{
            style: {
              background: 'var(--background)',
              color: 'var(--foreground)',
              border: '1px solid var(--border)',
            }
          }}
        />
      </ServerProvider>
    </ThemeProvider>
  )
}

export default AboutWindow