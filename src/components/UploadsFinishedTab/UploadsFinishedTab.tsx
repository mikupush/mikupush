import { useContext } from 'react'
import FileIcon from '../FileIcon/FileIcon'
import { UploadsContext } from '../../context/upload'
import styles from './UploadsFinishedTab.module.css'
import { Upload } from '../../model/upload'

export default function UploadsFinishedTab() {
	const { finishedUploads } = useContext(UploadsContext)

	if (finishedUploads.length > 0) {
		return <FinishedUploadList items={finishedUploads} />
	} else {
		return <EmptyState />
	}
}

interface FinishedUploadListProps {
    items: Upload[]
}

function FinishedUploadList({ items }: FinishedUploadListProps) {
	return (
		<md-list className={styles.list}>
			{items.map((upload, index) => (
				<UploadItemWithDivider
					key={index}
					upload={upload}
					index={index}
					totalItems={items.length}
				/>
			))}
		</md-list>
	)
}

interface UploadItemWithDividerProps {
    index: number
    upload: Upload
    totalItems: number
}

function UploadItemWithDivider({ index, upload, totalItems }: UploadItemWithDividerProps) {
	return (
		<>
			<UploadItem upload={upload} />
			{index < totalItems - 1 ? (
				<md-divider key={`divider-${index}`} />
			) : (
				''
			)}
		</>
	)
}

interface UploadItemProps {
    upload: Upload
}

function UploadItem({ upload }: UploadItemProps) {
	const { deleteUpload } = useContext(UploadsContext)

	const handleCopyLink = () => {
		//window.uploadChannels.copyLink(upload.id)
    console.log('copy link click')
	}

	const handleDelete = () => {
		deleteUpload(upload.id)
	}

	return (
		<md-list-item>
			<div slot="start">
				<FileIcon mimeType={upload.mimeType} />
			</div>

			<div slot="headline" className={styles.name}>
				{upload.name}
			</div>
			<div slot="supporting-text" className={styles.uploadedAt}>
				{''}
			</div>
			<div slot="end" className={styles.actions}>
				<md-icon-button onClick={handleCopyLink} className={styles.copyLink}>
					<md-icon>link</md-icon>
				</md-icon-button>
				<md-icon-button onClick={handleDelete}>
					<md-icon className={styles.cancel}>delete</md-icon>
				</md-icon-button>
			</div>
		</md-list-item>
	)
}

function EmptyState() {
	return (
		<div className={styles.emptyState}>
			<md-icon>inventory_2</md-icon>
			<p className="md-typescale-body-large">No files uploaded yet. Try uploading one!</p>
		</div>
	)
}
