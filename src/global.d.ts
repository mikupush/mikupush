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
