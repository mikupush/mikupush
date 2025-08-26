import {useUploadsStore} from "@/store/uploads.ts";
import { invoke } from "@tauri-apps/api/core";
import {UploadRequest} from "@/model/upload.ts";

export function cancelUpload(uploadId: string) {
    console.log('cancel upload')
    const store = useUploadsStore.getState()

    invoke<UploadRequest[]>('cancel_upload', { uploadId })
        .then((result) => store.setInProgressUploads(result))
        .catch((error) => console.warn(error))
}
