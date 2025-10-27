import { Toaster } from 'react-hot-toast'

export function ToastContainer() {
  return (
    <Toaster
      position="bottom-right"
      toastOptions={{
        style: {
          background: 'var(--background)',
            color: 'var(--foreground)',
            border: '1px solid var(--border)',
        }
      }}
    />
  )
}