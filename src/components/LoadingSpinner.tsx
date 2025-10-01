import { LoaderCircle } from 'lucide-react'

interface LoadingSpinnerProps {
  size?: number
}

export default function LoadingSpinner({ size }: LoadingSpinnerProps) {
  return (
    <div className="flex items-center justify-center size-full">
      <LoaderCircle size={size} className="animate-spin" />
    </div>
  )
}