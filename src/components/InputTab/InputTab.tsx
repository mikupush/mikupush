import {ChangeEvent, useRef} from 'react'
import styles from './InputTab.module.css'
import './InputTab.css'

export default function InputTab() {
	const fileInputRef = useRef<HTMLInputElement>(null)

	const openFileDialog = () => {
		if (fileInputRef.current != null) {
			fileInputRef.current.click()
		}
	}

	const handleSelectedFiles = (event: ChangeEvent<HTMLInputElement>) => {
		event.preventDefault()
		event.stopPropagation()

		console.log(event)
	}

	return (
		<div
			className={`${styles.area} file-drop-area`}
			onClick={openFileDialog}
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
