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