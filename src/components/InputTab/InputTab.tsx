import { useContext, useState, useRef, DragEvent, ChangeEvent } from 'react'
import styles from './InputTab.module.css'
import { UploadsContext } from '../../context/upload'

export default function InputTab() {
	const [active, setActive] = useState(false)
	const fileInputRef = useRef<HTMLInputElement>(null)
	const { requestUploads } = useContext(UploadsContext)

	const openFileDialog = () => {
		if (fileInputRef.current != null) {
			fileInputRef.current.click()
		}
	}

	const handleDragOver = (event: DragEvent<HTMLDivElement>) => {
		event.preventDefault()
		event.stopPropagation()

		setActive(true)
	}

	const handleDragLeave = (event: DragEvent<HTMLDivElement>) => {
		event.preventDefault()
		event.stopPropagation()

		setActive(false)
	}

	const handleDrop = (event: DragEvent<HTMLDivElement>) => {
		event.preventDefault()
		event.stopPropagation()

		const isFile = (item: DataTransferItem) => {
			if (item.kind !== 'file') {
				return false
			}

			const entry = item.webkitGetAsEntry()
			return entry?.isFile ?? false
		}

		if (event.dataTransfer == null) {
			return
		}

		const files = Array.from(event.dataTransfer.items)
			.filter(isFile)
			.map((item) => item.getAsFile())
			.filter((item) => item !== null)

		requestUploads(files)
		setActive(false)
	}

	const handleSelectedFiles = (event: ChangeEvent<HTMLInputElement>) => {
		event.preventDefault()
		event.stopPropagation()

		const input = event.target

		if (input instanceof HTMLInputElement && input.files != null) {
			requestUploads(Array.from(input.files))
			setActive(false)
		}
	}

	return (
		<div
			className={`${styles.area} ${active ? styles.active : ''}`}
			onClick={openFileDialog}
			onDragOver={handleDragOver}
			onDragEnter={handleDragOver}
			onDragLeave={handleDragLeave}
			onDrop={handleDrop}
		>
			<md-icon>upload</md-icon>
			<p className="md-typescale-body-large">
        Drop your file here to upload it, or click to select a file.
			</p>
			<input
				type="file"
				hidden
				onChange={handleSelectedFiles}
				ref={fileInputRef}
			/>
		</div>
	)
}
