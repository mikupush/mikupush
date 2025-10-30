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

import { v4 as uuidv4 } from 'uuid'

export interface Server {
  id: string;
  url: string;
  name: string;
  icon: string | null;
  alias: string | null;
  addedAt: string;
  testing: boolean;
  connected: boolean;
  healthy: boolean;
}

export const undefinedServer: Server = {
  id: '',
  url: '',
  name: '',
  icon: null,
  alias: null,
  addedAt: '',
  testing: false,
  connected: false,
  healthy: false
}

export function createServerFromUrl(url: string): Server {
  const urlObject = new URL(url)
  const hostname = urlObject.hostname

  return {
    id: uuidv4(),
    url: url,
    name: hostname,
    icon: null,
    alias: null,
    addedAt: new Date().toISOString(),
    testing: false,
    connected: false,
    healthy: false
  }
}

export class ServerNotFoundError extends Error {}