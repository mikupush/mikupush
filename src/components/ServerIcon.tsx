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
import { ServerIcon as LucideServerIcon } from 'lucide-react'
import { invoke } from '@tauri-apps/api/core'
import { useEffect, useState } from 'react'
import { cn } from '@/lib/utils.ts'

interface ServerIconProps {
  icon: string | null
  className?: string
}

export function ServerIcon({ icon, className }: Partial<ServerIconProps>) {
  const [base64Icon, setBase64Icon] = useState<string | null>(null)

  useEffect(() => {
    if (!icon) {
      setBase64Icon(null)
      return
    }

    if (icon.startsWith('data:')) {
      setBase64Icon(icon)
      return
    }

    invoke<string>('server_icon_url', { icon })
      .then(base64 => setBase64Icon(base64))
      .catch(() => setBase64Icon(null))
  }, [icon])

  const classes = cn(
    'size-20 flex justify-center items-center rounded-lg overflow-hidden',
    className
  )

  return (
    <div className={classes}>
      {base64Icon ? (
        <img className="h-full object-cover" src={base64Icon} alt="" />
      ) : (
        <LucideServerIcon className="h-full w-full" />
      )}
    </div>
  )
}
