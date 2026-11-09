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

import { cn } from '@/lib/utils'
import {
  File, FileArchive, FileAudio, FileCode, FileImage, FileSpreadsheet,
  FileText, FileVideo, FolderIcon, HardDrive, Presentation,
} from 'lucide-react'

interface FileIconProps {
  extension: string
  thumbnail?: string
  directory?: boolean
}

export default function FileIcon({ extension, directory }: FileIconProps) {
  const types = [
    { extensions: ['jpg', 'jpeg', 'png', 'gif', 'webp', 'avif', 'svg', 'bmp', 'ico', 'tif', 'tiff', 'heic'], icon: FileImage, color: 'text-file-image' },
    { extensions: ['mp4', 'mov', 'mkv', 'webm', 'avi', 'm4v', 'mpeg'], icon: FileVideo, color: 'text-file-video' },
    { extensions: ['mp3', 'wav', 'flac', 'ogg', 'aac', 'm4a', 'opus', 'aiff'], icon: FileAudio, color: 'text-file-audio' },
    { extensions: ['zip', 'rar', '7z', 'tar', 'gz', 'bz2', 'xz', 'tgz'], icon: FileArchive, color: 'text-file-archive' },
    { extensions: ['pdf'], icon: FileText, color: 'text-destructive' },
    { extensions: ['doc', 'docx', 'odt', 'txt', 'md', 'rtf'], icon: FileText, color: 'text-file-image' },
    { extensions: ['xls', 'xlsx', 'ods', 'csv'], icon: FileSpreadsheet, color: 'text-file-spreadsheet' },
    { extensions: ['ppt', 'pptx', 'odp'], icon: Presentation, color: 'text-file-archive' },
    { extensions: ['js', 'jsx', 'ts', 'tsx', 'html', 'css', 'scss', 'rs', 'py', 'java', 'json', 'yaml', 'yml', 'sh', 'toml', 'xml'], icon: FileCode, color: 'text-file-audio' },
    { extensions: ['dmg', 'iso', 'img'], icon: HardDrive, color: 'text-muted-foreground' },
  ]
  const type = types.find(type => type.extensions.includes(extension.toLowerCase().replace(/^\./, '')))
  const Icon = directory ? FolderIcon : type?.icon ?? File
  const color = directory ? 'text-file-archive' : type?.color ?? 'text-muted-foreground'

  return <Icon aria-hidden="true" className={cn('size-9 shrink-0', color)} strokeWidth={1.5} />
}
