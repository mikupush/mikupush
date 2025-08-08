import {
	faFile,
	faFileZipper,
	faFileCode,
	faFileLines,
	faFilePdf,
	faFilePowerpoint,
	faFileWord,
	faFileExcel,
	faFileVideo,
	faFileAudio,
	faFileImage,
} from '@fortawesome/free-solid-svg-icons'
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import styles from './FileIcon.module.css'

const excelTypes = [
	'application/vnd.ms-excel',
	'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
	'application/vnd.oasis.opendocument.spreadsheet',
]

const wordTypes = [
	'application/msword',
	'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
	'application/rtf',
	'application/vnd.oasis.opendocument.tex',
]

const powerPointTypes = [
	'application/vnd.ms-powerpoint',
	'application/vnd.openxmlformats-officedocument.presentationml.presentation',
	'application/vnd.oasis.opendocument.presentation',
]

const sourceCodeTypes = [
	'text/x-java-source',
	'text/x-c',
	'text/x-c++',
	'text/x-csharp',
	'text/x-python',
	'application/javascript',
	'application/x-typescript',
	'text/html',
	'text/css',
	'application/x-httpd-php',
	'text/x-ruby',
	'text/x-perl',
	'application/x-sh',
	'application/x-powershell',
	'application/sql',
	'text/x-go',
	'text/x-rust',
	'text/x-swift',
	'text/x-kotlin',
	'text/x-lua',
	'text/x-r-source',
	'text/x-matlab',
]

const compressedTypes = [
	'application/zip',
	'application/x-7z-compressed',
	'application/x-rar-compressed',
	'application/gzip',
	'application/x-tar',
	'application/x-bzip2',
	'application/x-xz',
	'application/x-lzma',
	'application/x-apple-diskimage',
]

interface FileIconProps {
  mimeType: string
}

export default function FileIcon({ mimeType }: FileIconProps) {
	let icon = faFile

	if (mimeType.startsWith('image/')) {
		icon = faFileImage
	} else if (mimeType.startsWith('audio/')) {
		icon = faFileAudio
	} else if (mimeType.startsWith('video/')) {
		icon = faFileVideo
	} else if (excelTypes.indexOf(mimeType) !== -1) {
		icon = faFileExcel
	} else if (wordTypes.indexOf(mimeType) !== -1) {
		icon = faFileWord
	} else if (powerPointTypes.indexOf(mimeType) !== -1) {
		icon = faFilePowerpoint
	} else if (mimeType === 'application/pdf') {
		icon = faFilePdf
	} else if (mimeType === 'text/plain') {
		icon = faFileLines
	} else if (sourceCodeTypes.indexOf(mimeType) !== -1) {
		icon = faFileCode
	} else if (compressedTypes.indexOf(mimeType) !== -1) {
		icon = faFileZipper
	}

	return <FontAwesomeIcon className={styles.icon} icon={icon} />
}
