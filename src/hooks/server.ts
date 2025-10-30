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

import { Server } from '@/model/server.ts'
import { useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/core'

const defaultServerIcon = 'data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNCIgaGVpZ2h0PSIyNCIgdmlld0JveD0iMCAwIDI0IDI0IiBmaWxsPSJub25lIiBzdHJva2U9ImN1cnJlbnRDb2xvciIgc3Ryb2tlLXdpZHRoPSIyIiBzdHJva2UtbGluZWNhcD0icm91bmQiIHN0cm9rZS1saW5lam9pbj0icm91bmQiIGNsYXNzPSJsdWNpZGUgbHVjaWRlLXNlcnZlci1pY29uIGx1Y2lkZS1zZXJ2ZXIiPjxyZWN0IHdpZHRoPSIyMCIgaGVpZ2h0PSI4IiB4PSIyIiB5PSIyIiByeD0iMiIgcnk9IjIiLz48cmVjdCB3aWR0aD0iMjAiIGhlaWdodD0iOCIgeD0iMiIgeT0iMTQiIHJ4PSIyIiByeT0iMiIvPjxsaW5lIHgxPSI2IiB4Mj0iNi4wMSIgeTE9IjYiIHkyPSI2Ii8+PGxpbmUgeDE9IjYiIHgyPSI2LjAxIiB5MT0iMTgiIHkyPSIxOCIvPjwvc3ZnPg=='

export function useServerIcon(server: Server) {
  const [icon, setIcon] = useState(defaultServerIcon)

  useEffect(() => {
    if (!server.icon) {
      setIcon(defaultServerIcon)
      return
    }

    invoke<string>('server_icon_url', { icon: server.icon })
      .then(base64 => setIcon(base64))
  }, [server])

  return icon
}