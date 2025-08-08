import { SerializableUpload, Upload } from './upload'
import { UploadProgress, SerializableUploadProgress } from './upload-progress'
import { FileDetails } from './file-details.ts'

export interface SerializableUploadRequest {
    file: FileDetails
    upload: SerializableUpload
    progress: SerializableUploadProgress
    retry?: boolean
}

export class UploadRequest {
	public readonly file: FileDetails
	public readonly upload: Upload
	private readonly _progress: UploadProgress
	private _controller: AbortController
	private _retry: boolean

	constructor(params: {
		file: FileDetails
		upload: Upload
		progress: UploadProgress
		controller?: AbortController
		retry?: boolean
	}) {
		this.file = params.file
		this.upload = params.upload
		this._controller = params.controller ?? new AbortController()
		this._progress = params.progress
		this._retry = params.retry ?? false
	}

	get id() {
		return this.upload.id
	}

	get name() {
		return this.upload.name
	}

	get mimeType() {
		return this.upload.mimeType
	}

	get progress() {
		return this._progress.progress
	}

	get speed() {
		return this._progress.speed
	}

	get error() {
		return this._progress.error
	}

	get isInProgress() {
		return this._progress.isInProgress
	}

	get finishedSuccess() {
		return this._progress.finishedSuccess
	}

	get finishedFailed() {
		return this._progress.finishedFailed
	}

	get controller() {
		return this._controller
	}

	get isRetried() {
		return this._retry
	}

	updateProgress(progress: number, speed: number) {
		this._progress.update(progress, speed)
	}

	finishWithError(error: Error | string | unknown) {
		this._progress.finishWithError(error)
	}

	finishSuccess() {
		this._progress.finishSuccess()
	}

	abort() {
		if (this.controller != null) {
			this.controller.abort()
		}
	}

	retry() {
		this._controller = new AbortController()
		this._retry = true
	}

	toSerializable(): SerializableUploadRequest {
		return {
			file: this.file,
			upload: this.upload.toSerializable(),
			progress: this._progress.toSerializable(),
			retry: this._retry,
		}
	}

	static async fromFileDetails(file: FileDetails) {
		return new UploadRequest({
			file,
			upload: await Upload.fromFileDetails(file),
			progress: UploadProgress.create()
		})
	}

	static fromSerializable(serializable: SerializableUploadRequest) {
		return new UploadRequest({
			file: serializable.file,
			upload: Upload.fromSerializable(serializable.upload),
			progress: UploadProgress.fromSerializable(serializable.progress),
			retry: serializable.retry ?? false
		})
	}
}
