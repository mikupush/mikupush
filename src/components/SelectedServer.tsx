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
import appIcon from '@/assets/app-icon.png'
import { Small } from '@/components/Typography'

export default function SelectedServer() {
  return (
    <div className="flex items-center space-x-[10px]">
      <div className="flex items-center justify-center size-[30px] overflow-hidden">
        <img className="h-full" src={appIcon} alt="mikupush.io" />
      </div>
      <Small weight="semibold">mikupush.io</Small>
    </div>
  )
}
