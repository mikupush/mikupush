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

export function formatRate(rateBytes: number): string {
  const maxBytes = 1024
  const maxKb = 1024 * 1024
  const maxMb = 1024 * 1024 * 1024

  if (rateBytes < maxBytes) {
    return `${rateBytes} B/s`
  }

  if (rateBytes < maxKb) {
    return `${Math.round(rateBytes / 1024)} KB/s`
  }

  if (rateBytes < maxMb) {
    return `${(rateBytes / (1024 * 1024)).toFixed(1)} MB/s`
  }

  return `${(rateBytes / (1024 * 1024 * 1024)).toFixed(1)} GB/s`
}

export function formatSizeBytes(bytes: number): string {
  const maxBytes = 1024
  const maxKb = 1024 * 1024
  const maxMb = 1024 * 1024 * 1024

  if (bytes < maxBytes) {
    return `${bytes} B`
  }

  if (bytes < maxKb) {
    return `${Math.round(bytes / 1024)} KB`
  }

  if (bytes < maxMb) {
    return `${(bytes / (1024 * 1024)).toFixed(1)} MB`
  }

  return `${(bytes / (1024 * 1024 * 1024)).toFixed(1)} GB`
}

export function formatDate(date: string | Date) {
  let dateTime: Date

  if (typeof date === 'string') {
    dateTime = new Date(date)
  } else {
    dateTime = date
  }

  if (isNaN(dateTime.getTime())) {
    return ''
  }

  const year = dateTime.getFullYear()
  const month = String(dateTime.getMonth() + 1).padStart(2, '0')
  const day = String(dateTime.getDate()).padStart(2, '0')
  const hour = String(dateTime.getHours()).padStart(2, '0')
  const minute = String(dateTime.getMinutes()).padStart(2, '0')

  return `${year}-${month}-${day} ${hour}:${minute}`
}