import { useEffect, useState } from 'react'
import styles from './App.module.css'
import { UploadsContext } from './context/upload'
import { Upload } from './model/upload'
import { getCurrentWebview } from '@tauri-apps/api/webview'
import { invoke } from '@tauri-apps/api/core'
import AppBar from '@/components/AppBar'
import Uploads from '@/components/Uploads.tsx'
import { Toaster } from 'react-hot-toast'

await getCurrentWebview().onDragDropEvent(async (event) => {
	const dropArea = document.querySelectorAll('.file-drop-area');

	const showActive = () => dropArea.forEach(el => {
		el.classList.remove('active')
		el.classList.add('active')
	})

	const hideActive = () => dropArea.forEach(el => {
		el.classList.remove('active')
	})
	// añadir aqui clase de css al input del fichero y llamar a un comando de tauri
	// se repiten mucho los eventos habra que tener cuidado con eso
	// https://v2.tauri.app/plugin/dialog/#build-a-file-selector-dialog
	if (event.payload.type === 'over') {
		showActive()
	} else if (event.payload.type === 'drop') {
		await invoke('enqueue_many_uploads', { paths: event.payload.paths })
	} else {
		hideActive()
	}
})

function App() {
	/* const [inProgressUploads, setInProgressUploads] = useState<UploadRequest[]>([])
	const [finishedUploads, setFinishedUploads] = useState<Upload[]>([])
	const [inProgressUploadsCount, setInProgressUploadsCount] = useState(0)
	const [finishedUploadsCount, setFinishedUploadsCount] = useState(0)

	useEffect(() => {
		// uploadChannels.findAll().then(uploads => {
		// 	setFinishedUploads(uploads.map(upload => Upload.fromSerializable(upload)))
		// })
	}, [])

	const moveRequestAsFinished = (request: UploadRequest) => {
		const isInFinishedUploads = finishedUploads.find(item => item.id === request.id) !== undefined

		if (isInFinishedUploads) {
			return
		}

		setInProgressUploads(inProgressUploads.filter((item) => item.id !== request.id))
		setInProgressUploadsCount(inProgressUploads.length)
		setFinishedUploads([request.upload, ...finishedUploads])

		// systemChannels.showNotification({
		// 	title: `The file ${request.name} has been uploaded!`,
		// 	body: 'Now, you can grab the link for share it!'
		// })
	}

  // @ts-ignore
	const handleProgressUpdate = (serializable: SerializableUploadRequest) => {
		const request = UploadRequest.fromSerializable(serializable)

		if (request.finishedSuccess) {
			moveRequestAsFinished(request)
		} else {
			setInProgressUploads((previous) => previous.map((item) => item.id === request.id ? request : item))
		}
	}

	const requestUploads = async (files: File[]) => {
		let newUploads: UploadRequest[] = []

		try {
      console.log('request upload', files)
			// const filePaths = files.map((file) => systemChannels.resolveWebFilePath(file))
      //
			// newUploads = await uploadChannels.enqueue(filePaths)
			// 	.then(requests => requests.map(request => UploadRequest.fromSerializable(request)))
      //
			// uploadChannels.onUploadProgress(handleProgressUpdate)
		} catch (exception) {
			console.error('unable to upload file', exception)
		}

		setInProgressUploads((previous) => [...previous, ...newUploads])
	}

	const cancelUpload = (request: UploadRequest) => {
		// uploadChannels.abort(request.id)
		setInProgressUploads(inProgressUploads.filter((item) => item.id !== request.id))
	}

	const retryUpload = (request: UploadRequest) => {
    console.log('retry upload', request)
		// uploadChannels.retry(request.toSerializable())
		// uploadChannels.onUploadProgress(handleProgressUpdate)
	}

	const resetInProgressUploadsCount = () => setInProgressUploadsCount(0)
	const resetFinishedUploadsCount = () => setFinishedUploadsCount(0)

	const deleteUpload = async (id: string) => {
		// await uploadChannels.delete(id)

		setFinishedUploads((previous) =>
			previous.filter((item) => item.id !== id)
		)
	} */

	return (
		<div className={styles.app}>
      <AppBar />
			<Uploads />
      <Toaster position="bottom-right" />
		</div>
	)
}

export default App
