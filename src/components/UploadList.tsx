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
import { UploadProgressItem, UploadItem } from '@/components/UploadItem'
import { UploadRequest } from '@/model/upload'

interface UploadListProps {
  items: UploadRequest[]
}

export default function UploadList({ items }: UploadListProps) {
  return (
    <div className="overflow-y-scroll flex-1">
      {items.map(item => <UploadItem key={item.upload.id} item={item} />)}
    </div>
  )
}

export function UploadProgressList({ items }: UploadListProps) {
  return (
    <div className="overflow-y-scroll flex-1">
      {items.map(item => <UploadProgressItem key={item.upload.id} item={item} />)}
    </div>
  )
}
