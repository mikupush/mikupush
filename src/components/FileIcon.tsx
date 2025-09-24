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

import { Large } from '@/components/Typography'

interface FileIconProps {
  extension: string
  thumbnail?: string
}

export default function FileIcon(props: FileIconProps) {
  const extension = (props.extension !== '') ? props.extension : '?'

	return (
    <div className="flex items-center justify-center rounded-xl bg-gray-400 w-[80px] h-[80px]">
      <Large align="center" className="text-white uppercase">{extension}</Large>
    </div>
  )
}