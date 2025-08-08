import { useContext } from 'react'
import FileIcon from '../FileIcon/FileIcon'
import { UploadsContext } from '../../context/upload'
import styles from './UploadsProgressTab.module.css'
import { UploadRequest } from '../../model/upload-request'

export default function UploadsProgressTab() {
	const { inProgressUploads } = useContext(UploadsContext)

	if (inProgressUploads.length > 0) {
		return <UploadsProgressList items={inProgressUploads} />
	} else {
		return <EmptyState />
	}
}

interface UploadsProgressListProps {
    items: UploadRequest[]
}

function UploadsProgressList({ items }: UploadsProgressListProps) {
	return (
		<md-list className={styles.list}>
			{items.map((upload, index) => (
				<UploadProgressItemWithDivider
					key={index}
					upload={upload}
					index={index}
					totalItems={items.length}
				/>
			))}
		</md-list>
	)
}

interface UploadProgressItemWithDividerProps {
    index: number
    upload: UploadRequest
    totalItems: number
}

function UploadProgressItemWithDivider({ index, upload, totalItems }: UploadProgressItemWithDividerProps) {
	const { cancelUpload, retryUpload } = useContext(UploadsContext)

	return (
		<>
			<UploadProgressItem
				upload={upload}
				onRetry={() => retryUpload(upload)}
				onCancel={() => cancelUpload(upload)}
			/>
			{index < totalItems - 1 ? (
				<md-divider key={`divider-${index}`} />
			) : (
				''
			)}
		</>
	)
}

interface UploadProgressItemProps {
    upload: UploadRequest
    onRetry: () => void
    onCancel: () => void
}

function UploadProgressItem({ upload, onRetry, onCancel }: UploadProgressItemProps) {
	return (
		<md-list-item>
			<div slot="start">
				<FileIcon mimeType={upload.mimeType} />
			</div>

			{upload.isInProgress ? (
				<InProgress upload={upload} onCancel={onCancel} />
			) : (
				''
			)}

			{upload.finishedFailed ? (
				<Error upload={upload} onRetry={onRetry} onCancel={onCancel} />
			) : (
				''
			)}
		</md-list-item>
	)
}

interface InProgressProps {
    upload: UploadRequest
    onCancel: () => void
}

function InProgress({ upload, onCancel }: InProgressProps) {
	const formatSpeed = (speed: number | undefined) => {
		if (typeof speed === 'undefined' || speed <= 0) {
			return '0 B/s'
		}

		const kb = speed / 1024
		const mb = kb / 1024
		const gb = mb / 1024

		if (gb >= 1) return `${gb.toFixed(0)} GB/s`
		if (mb >= 1) return `${mb.toFixed(0)} MB/s`
		if (kb >= 1) return `${kb.toFixed(0)} KB/s`
		return `${speed.toFixed(0)} B/s`
	}

	return (
		<>
			<div slot="headline" className={styles.name}>
				{upload.name}
			</div>
			<div slot="supporting-text" className={styles.speed}>
				{formatSpeed(upload.speed)}
			</div>
			<div slot="end" className={styles.actions}>
				<md-circular-progress
					className={styles.progress}
					value={upload.progress}
				/>
				<md-icon-button onClick={onCancel}>
					<md-icon className={styles.cancel}>close</md-icon>
				</md-icon-button>
			</div>
		</>
	)
}

interface ErrorProps {
    upload: UploadRequest
    onRetry: () => void
    onCancel: () => void
}

function Error({ upload, onRetry, onCancel }: ErrorProps) {
	return (
		<>
			<div slot="headline" className={styles.name}>
				{upload.name}
			</div>
			<div slot="supporting-text" className={styles.error}>
				{upload.error}
			</div>
			<div slot="end" className={styles.actions}>
				<md-icon-button onClick={onRetry}>
					<md-icon className={styles.retry}>refresh</md-icon>
				</md-icon-button>
				<md-icon-button onClick={onCancel}>
					<md-icon className={styles.cancel}>close</md-icon>
				</md-icon-button>
			</div>
		</>
	)
}

function EmptyState() {
	return (
		<div className={styles.emptyState}>
			<md-icon>inventory_2</md-icon>
			<p className="md-typescale-body-large">No uploads in progress yet. Try uploading a file!</p>
		</div>
	)
}
