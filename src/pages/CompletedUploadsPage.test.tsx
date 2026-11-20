import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react'
import { invoke } from '@tauri-apps/api/core'
import { afterEach, expect, it, vi } from 'vitest'
import CompletedUploadsPage from '@/pages/CompletedUploadsPage'
import { type Upload } from '@/model/upload'

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn() }))
vi.mock('@tauri-apps/api/event', () => ({ listen: vi.fn().mockResolvedValue(vi.fn()) }))
vi.mock('@/hooks/server.ts', () => ({ useServerConnector: () => ({ current: { id: 'server-1' } }) }))
vi.mock('react-i18next', () => ({ useTranslation: () => ({ t: (key: string) => key }) }))

afterEach(() => {
  cleanup()
  vi.clearAllMocks()
})

it('runs a manual status check for the selected server and disables the button while waiting', async () => {
  const upload: Upload = {
    id: 'upload-1',
    name: 'file.txt',
    size: 100,
    mimeType: 'text/plain',
    path: '/tmp/file.txt',
    createdAt: '2026-11-09T12:00:00Z',
    status: 'completed',
    serverId: 'server-1',
    directory: false,
  }
  let finishSync!: (ran: boolean) => void
  const sync = new Promise<boolean>((resolve) => {
    finishSync = resolve
  })
  vi.mocked(invoke).mockImplementation(async (command) => {
    if (command === 'get_archived_uploads') return [upload]
    if (command === 'sync_archived_upload_statuses') return sync
  })
  render(<CompletedUploadsPage />)

  const button = await screen.findByRole<HTMLButtonElement>('button', { name: 'uploads.status_sync.action' })
  fireEvent.click(button)
  expect(invoke).toHaveBeenCalledWith('sync_archived_upload_statuses', { serverId: 'server-1' })
  expect(button.disabled).toBe(true)

  finishSync(true)
  await waitFor(() => expect(button.disabled).toBe(false))
})
