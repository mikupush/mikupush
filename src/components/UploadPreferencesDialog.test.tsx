import { act, cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react'
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { invoke } from '@tauri-apps/api/core'
import { UploadPreferencesDialog } from '@/components/UploadPreferencesDialog'
import { useUploadsStore } from '@/store/uploads'
import { requestUploads, startUploads } from '@/helpers/upload.ts'
import { selectFiles, selectFolders } from '@/helpers/file'
import { CONFIG_ASK_UPLOAD_PREFERENCES, CONFIG_UPLOAD_PREFERENCES } from '@/constants/config'
import { UploadRequest } from '@/model/upload'

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn() }))
vi.mock('react-i18next', () => ({ useTranslation: () => ({ t: translate }) }))
const translate = (key: string) => key
const ipc = vi.mocked(invoke)
let config: Record<string, string>
let selection: string[]
const uploads: UploadRequest[] = []

beforeEach(() => {
  config = { [CONFIG_ASK_UPLOAD_PREFERENCES]: 'false', [CONFIG_UPLOAD_PREFERENCES]: '{"encryption":false}' }
  selection = ['/tmp/file.txt']
  uploads.length = 0
  useUploadsStore.setState({ preferenceDialogs: [], inProgressUploads: [] })
  ipc.mockImplementation(async (command, args) => {
    const data = args as Record<string, unknown>
    if (command === 'get_config_value') return config[data.key as string]
    if (command === 'set_config_value') { config[data.key as string] = data.value as string; return }
    if (command.startsWith('select_')) return selection
    if (command === 'enqueue_uploads') {
      uploads.push({ preferences: structuredClone(data.preferences) } as UploadRequest)
      return [...uploads]
    }
    throw new Error(`Unexpected command: ${command}`)
  })
})
afterEach(cleanup)

async function openDialog(paths?: string[]) {
  act(() => useUploadsStore.getState().queueUploadPreferences(paths))
  render(<UploadPreferencesDialog />)
  await waitFor(() => expect((screen.getByRole('switch') as HTMLButtonElement).disabled).toBe(false))
}

function uploadCalls() {
  return ipc.mock.calls.filter(([command]) => command === 'enqueue_uploads')
}

describe('upload preferences flow', () => {
  it.each([selectFiles, selectFolders])('selects files/folders without uploading until confirmed', async select => {
    config[CONFIG_ASK_UPLOAD_PREFERENCES] = 'true'
    render(<UploadPreferencesDialog />)
    act(() => select())
    await waitFor(() => expect((screen.getByRole('switch') as HTMLButtonElement).disabled).toBe(false))
    expect(uploadCalls()).toHaveLength(0)
    fireEvent.click(screen.getByRole('switch'))
    fireEvent.click(screen.getByText('uploads.preferences.start'))
    await waitFor(() => expect(screen.queryByRole('dialog')).toBeNull())
    expect(uploadCalls()).toHaveLength(1)
    expect(uploadCalls()[0][1]).toEqual({ paths: selection, preferences: { encryption: true } })
    // One-off changes do not replace saved defaults.
    expect(config[CONFIG_UPLOAD_PREFERENCES]).toBe('{"encryption":false}')
  })

  it.each(['button', 'escape', 'close'])('cancels automatic preferences via %s without starting or saving', async method => {
    await openDialog(selection)
    fireEvent.click(screen.getByRole('switch'))
    if (method === 'button') fireEvent.click(screen.getByText('common.cancel'))
    if (method === 'escape') fireEvent.keyDown(screen.getByRole('dialog'), { key: 'Escape' })
    if (method === 'close') fireEvent.click(screen.getByRole('button', { name: 'Close' }))
    await waitFor(() => expect(screen.queryByRole('dialog')).toBeNull())
    expect(uploadCalls()).toHaveLength(0)
    expect(config[CONFIG_UPLOAD_PREFERENCES]).toBe('{"encryption":false}')
  })

  it('saves manual defaults for the next upload, preserving active transfers', async () => {
    await requestUploads(selection)
    expect(uploads[0].preferences.encryption).toBe(false)
    await openDialog()
    fireEvent.click(screen.getByRole('switch'))
    fireEvent.click(screen.getByText('uploads.preferences.save'))
    await waitFor(() => expect(screen.queryByRole('dialog')).toBeNull())
    expect(uploadCalls()).toHaveLength(1)
    expect(JSON.parse(config[CONFIG_UPLOAD_PREFERENCES])).toEqual({ encryption: true })
    await requestUploads(['/tmp/next.txt'])
    expect(uploads.map(upload => upload.preferences.encryption)).toEqual([false, true])
  })

  it('discards manual changes when canceled', async () => {
    await openDialog()
    fireEvent.click(screen.getByRole('switch'))
    fireEvent.click(screen.getByText('common.cancel'))
    await requestUploads(selection)
    expect(uploads[0].preferences.encryption).toBe(false)
  })

  it('keeps successive selections separate when the first is canceled', async () => {
    config[CONFIG_ASK_UPLOAD_PREFERENCES] = 'true'
    await requestUploads(['/tmp/first.txt'])
    await requestUploads(['/tmp/second.txt'])
    render(<UploadPreferencesDialog />)
    fireEvent.click(screen.getByText('common.cancel'))
    await waitFor(() => expect((screen.getByRole('switch') as HTMLButtonElement).disabled).toBe(false))
    fireEvent.click(screen.getByText('uploads.preferences.start'))
    await waitFor(() => expect(uploadCalls()).toHaveLength(1))
    expect(uploadCalls()[0][1]).toEqual({ paths: ['/tmp/second.txt'], preferences: { encryption: false } })
  })

  it('does nothing if the native selection is canceled', async () => {
    selection = []
    selectFiles()
    selectFolders()
    await waitFor(() => expect(ipc).toHaveBeenCalledTimes(2))
    expect(uploadCalls()).toHaveLength(0)
    expect(useUploadsStore.getState().preferenceDialogs).toHaveLength(0)
  })

  it('keeps the dialog open if persistence fails', async () => {
    await openDialog()
    ipc.mockRejectedValueOnce(new Error('SQLite unavailable'))
    fireEvent.click(screen.getByText('uploads.preferences.save'))
    await screen.findByRole('alert')
    expect(screen.getByRole('dialog')).toBeTruthy()
    expect(uploadCalls()).toHaveLength(0)
  })

  it('does not silently start an upload if reading configuration fails', async () => {
    ipc.mockRejectedValueOnce(new Error('SQLite unavailable'))
    await expect(requestUploads(selection)).rejects.toThrow('SQLite unavailable')
    expect(uploadCalls()).toHaveLength(0)
  })

  it('prevents duplicate confirmation while an upload is being enqueued', async () => {
    await openDialog(selection)
    let finish!: () => void
    ipc.mockImplementationOnce(() => new Promise(resolve => { finish = () => resolve([]) }))
    fireEvent.click(screen.getByText('uploads.preferences.start'))
    fireEvent.click(screen.getByText('uploads.preferences.start'))
    fireEvent.keyDown(screen.getByRole('dialog'), { key: 'Escape' })
    expect(uploadCalls()).toHaveLength(1)
    expect(screen.getByRole('dialog')).toBeTruthy()
    await act(async () => finish())
    expect(screen.queryByRole('dialog')).toBeNull()
  })

  it('leaves a failed upload submission open for correction or cancellation', async () => {
    await openDialog(selection)
    ipc.mockRejectedValueOnce(new Error('File no longer exists'))
    fireEvent.click(screen.getByText('uploads.preferences.start'))
    await screen.findByRole('alert')
    expect(screen.getByRole('dialog')).toBeTruthy()
    fireEvent.click(screen.getByText('common.cancel'))
    expect(screen.queryByRole('dialog')).toBeNull()
    expect(uploadCalls()).toHaveLength(1)
  })

  it('copies preferences before crossing the IPC boundary', async () => {
    const preferences = { encryption: true }
    const pending = startUploads(selection, preferences)
    preferences.encryption = false
    selection.push('/tmp/late.txt')
    await pending
    expect(uploadCalls()[0][1]).toEqual({ paths: ['/tmp/file.txt'], preferences: { encryption: true } })
  })
})
