import appIcon from '@/assets/app-icon.png'
import { Small } from '@/components/Typography'

export default function SelectedServer() {
  return (
    <div className="flex items-center space-x-[10px]">
      <div className="flex items-center justify-center size-[30px] overflow-hidden">
        <img className="h-full" src={appIcon} alt="mikupush.io" />
      </div>
      <Small weight="semibold">mikupush.io</Small>
    </div>
  )
}
