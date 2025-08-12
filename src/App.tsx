import { useEffect, useState } from 'react'
import styles from './App.module.css'
import AppTabs from './components/AppTabs/AppTabs'
import AppTitle from './components/AppTitle/AppTitle'
import InputTab from './components/InputTab/InputTab'
import UploadsFinishedTab from './components/UploadsFinishedTab/UploadsFinishedTab'
import UploadsProgressTab from './components/UploadsProgressTab/UploadsProgressTab'
import { UploadsContext } from './context/upload'
import { SerializableUploadRequest, UploadRequest } from './model/upload-request.ts'
import { Upload } from './model/upload.ts'
import { getCurrentWebview } from "@tauri-apps/api/webview";
import {invoke} from "@tauri-apps/api/core";

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
	const tabs = {
		upload: <InputTab />,
		'uploads-in-progress': <UploadsProgressTab />,
		'finished-uploads': <UploadsFinishedTab />,
	}

	const [currentTab, setCurrentTab] = useState<keyof typeof tabs>('upload')
	const [inProgressUploads, setInProgressUploads] = useState<UploadRequest[]>([])
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

		if (currentTab !== 'finished-uploads') {
			setFinishedUploadsCount(finishedUploadsCount + 1)
		}

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

		if (currentTab !== 'uploads-in-progress') {
			setInProgressUploadsCount((previous) => previous + newUploads.length)
		}
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

	const handleTabSelected = (index: keyof typeof tabs) => {
		setCurrentTab(index)
	}

	const deleteUpload = async (id: string) => {
		// await uploadChannels.delete(id)

		setFinishedUploads((previous) =>
			previous.filter((item) => item.id !== id)
		)
	}

	return (
		<UploadsContext.Provider value={{
			inProgressUploads,
			finishedUploads,
			inProgressUploadsCount,
			finishedUploadsCount,
			requestUploads,
			cancelUpload,
			retryUpload,
			resetInProgressUploadsCount,
			resetFinishedUploadsCount,
			deleteUpload
		}}>
			<div className={styles.app}>
				<div>
					<div className={styles.dragArea} />
					<AppTitle />
					<AppTabs onTabSelected={handleTabSelected} />
				</div>
				<div className={styles.content}>{tabs[currentTab]}</div>
			</div>
		</UploadsContext.Provider>
	)
}

export default App
