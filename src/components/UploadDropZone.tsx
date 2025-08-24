import { useTranslation } from "react-i18next"

export default function UploadDropZone() {
  const { t } = useTranslation()

  return (
    <div className="w-full h-full flex absolute inset-0 p-[10px]">
      <div className="flex flex-1 items-center justify-center rounded-3xl text-4xl font-extrabold bg-primary/95 text-primary-foreground">
        {t('uploads.drop')}
      </div>
    </div>
  )
}