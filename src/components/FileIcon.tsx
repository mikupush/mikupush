import { Large } from "@/components/Typography"

interface FileIconProps {
  extension: string
  thumbnail?: string
}

export default function FileIcon({ extension }: FileIconProps) {
	return (
    <div className="flex items-center justify-center rounded-xl bg-gray-400 w-[80px] h-[80px]">
      <Large align="center" className="text-white uppercase">{extension}</Large>
    </div>
  )
}
