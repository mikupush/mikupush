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

export const CONFIG_THEME = 'theme'
export const CONFIG_START_ON_SYSTEM_STARTUP = 'start_on_system_startup'
export const CONFIG_START_MINIMIZED = 'start_minimized'
export const CONFIG_UPLOAD_IN_CHUNKS = 'upload_in_chunks'
export const CONFIG_UPLOAD_CHUNK_SIZE = 'upload_chunk_size'

export type ConfigKey = typeof CONFIG_THEME
  | typeof CONFIG_START_ON_SYSTEM_STARTUP
  | typeof CONFIG_START_MINIMIZED
  | typeof CONFIG_UPLOAD_IN_CHUNKS
  | typeof CONFIG_UPLOAD_CHUNK_SIZE

export const CONFIG_TRUE_VALUE = 'true'
export const CONFIG_FALSE_VALUE = 'false'