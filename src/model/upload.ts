import { v4 as uuidv4 } from 'uuid'
import { FileDetails } from './file-details.ts'

export interface SerializableUpload {
    id: string
    name: string
    size: number
    mimeType: string
    uploadedAt: Date
}

export class Upload {
	public readonly id: string
	public readonly name: string
	public readonly size: number
	public readonly mimeType: string
	public readonly uploadedAt: Date

	constructor(params: {
		id: string
		name: string
		size: number
		mimeType: string
		uploadedAt: Date
	}) {
		this.id = params.id
		this.name = params.name
		this.size = params.size
		this.mimeType = params.mimeType
		this.uploadedAt = params.uploadedAt
	}

	toSerializable(): SerializableUpload {
		return {
			id: this.id,
			name: this.name,
			size: this.size,
			mimeType: this.mimeType,
			uploadedAt: this.uploadedAt
		}
	}

	static async fromFileDetails(file: FileDetails) {
		return new Upload({
			id: uuidv4(),
			name: file.name,
			size: file.size,
			mimeType: file.mimeType,
			uploadedAt: new Date()
		})
	}

	static fromSerializable(serializable: SerializableUpload) {
		return new Upload(serializable)
	}
}
