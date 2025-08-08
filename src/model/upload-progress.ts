export interface SerializableUploadProgress {
    progress: number
    speed: number
    error: string
    finished: boolean
}

export class UploadProgress {
	private _progress: number
	private _speed: number
	private _error: string
	private finished: boolean

	constructor(params: {
		progress: number
		speed: number
		error: string
		finished: boolean
	}) {
		this._progress = params.progress
		this._speed = params.speed
		this._error = params.error
		this.finished = params.finished
	}

	get progress() {
		return this._progress
	}

	get speed() {
		return this._speed
	}

	get error() {
		return this._error
	}

	get isInProgress() {
		return !this.finished
	}

	get finishedSuccess() {
		return this.finished && this.error === ''
	}

	get finishedFailed() {
		return this.finished && this.error !== ''
	}

	update(progress: number, speed: number) {
		this._progress = progress
		this._speed = speed
	}

	finishWithError(error: Error | string | unknown) {
		if (error instanceof Error) {
			this._error = error.message
		} else if (typeof error === 'string') {
			this._error = error
		} else {
			this._error = 'an unknown error occurred during upload'
		}

		this.finished = true
	}

	finishSuccess() {
		this._error = ''
		this.finished = true
	}

	toSerializable(): SerializableUploadProgress {
		return {
			progress: this.progress,
			speed: this.speed,
			error: this.error,
			finished: this.finished
		}
	}

	static create() {
		return new UploadProgress({
			progress: 0,
			speed: 0,
			error: '',
			finished: false
		})
	}

	static fromSerializable(serializable: SerializableUploadProgress) {
		return new UploadProgress(serializable)
	}
}
