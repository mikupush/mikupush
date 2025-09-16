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
import * as React from 'react'
import { ClipBoardAPI, NotificationAPI, UploadAPI } from '../shared/ipc.ts'

declare module 'react' {
    namespace JSX {
        interface IntrinsicElements {
            // Material Web Components type as normal html elements
            [elemName: `md-${string}`]: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>
            'md-circular-progress': React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement> & { value: number }
        }
    }
}

declare global {
	interface Window {
		uploadAPI: UploadAPI;
		clipBoardAPI: ClipBoardAPI;
		notificationAPI: NotificationAPI;
	}
}
