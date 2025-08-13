import SelectedServer from '@/components/SelectedServer'
import { Button } from '@/components/ui/button'
import { selectFiles } from '@/ipc'
import { UploadIcon, SidebarOpenIcon } from 'lucide-react'

export default function AppBar() {
  return (
    <div className="flex items-center space-x-[10px] m-[15px]">
      <SelectedServer />
      <Button variant="outline" size="icon">
        <SidebarOpenIcon />
      </Button>
      <Button
        variant="outline" 
        size="icon"
        onClick={() => selectFiles()}
      >
        <UploadIcon />
      </Button>
    </div>
  )
}