import { cleanup, fireEvent, render, screen, within } from '@testing-library/react'
import { afterEach, describe, expect, it, vi } from 'vitest'
import { invoke } from '@tauri-apps/api/core'
import { ArchivedUploadList, UploadProgressList } from '@/components/UploadList'
import { cancelUpload, retryUpload } from '@/helpers/upload'
import { UploadRequest } from '@/model/upload'

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn().mockResolvedValue([]) }))
vi.mock('@/helpers/upload', () => ({ cancelUpload: vi.fn(), retryUpload: vi.fn() }))
vi.mock('react-i18next', () => ({ useTranslation: () => ({ t: (key: string) => key }) }))

afterEach(() => {
  cleanup()
  vi.clearAllMocks()
})

function request(overrides: Partial<UploadRequest> = {}): UploadRequest {
  return {
    preferences: { encryption: true },
    progress: { progress: 0.427, uploadedBytes: 427, totalSize: 1000, rateBytes: 1024 },
    finished: false,
    canceled: false,
    upload: {
      id: 'upload-1', name: 'holiday.mp4', size: 1000, mimeType: 'video/mp4',
      path: '/tmp/holiday.mp4', createdAt: '2026-11-09T12:00:00Z',
      status: 'inProgress', serverId: 'server-1', directory: false,
    },
    ...overrides,
  }
}

describe('upload tables', () => {
  it('renders aligned columns with numeric progress and keeps cancel available', () => {
    render(<UploadProgressList items={[request()]} />)
    const table = screen.getByRole('table', { name: 'uploads.table.caption' })
    expect(within(table).getAllByRole('columnheader')).toHaveLength(7)
    expect(within(table).getAllByRole('cell')).toHaveLength(7)
    expect(screen.getByText('42%')).toBeTruthy()
    expect(screen.getByText('1 KB/s')).toBeTruthy()
    expect(screen.queryByRole('progressbar')).toBeNull()
    fireEvent.click(screen.getByRole('button', { name: 'uploads.actions.cancel' }))
    expect(cancelUpload).toHaveBeenCalledWith('upload-1')
  })

  it.each(['enqueued', 'pending', 'compressing'] as const)('shows %s separately from transfer progress', status => {
    const item = request()
    item.upload.status = status
    render(<UploadProgressList items={[item]} />)
    expect(screen.getByText(`uploads.status.${status}`)).toBeTruthy()
    expect(screen.getByText('0%')).toBeTruthy()
    expect(screen.queryByText('1 KB/s')).toBeNull()
  })

  it('does not show 100% until the upload has finished', () => {
    const item = request()
    item.progress.progress = 1
    const view = render(<UploadProgressList items={[item]} />)
    expect(screen.getByText('99%')).toBeTruthy()
    view.rerender(<UploadProgressList items={[{ ...item, finished: true }]} />)
    expect(screen.getByText('100%')).toBeTruthy()
    expect(screen.getByText('uploads.status.completed')).toBeTruthy()
  })

  it('keeps failures readable and retryable', () => {
    render(<UploadProgressList items={[request({ finished: true, error: { code: 'client_error', message: 'Offline' } })]} />)
    expect(screen.getByText('uploads.status.failed')).toBeTruthy()
    expect(screen.getByText('errors.upload.client_error')).toBeTruthy()
    expect(screen.getByText('42%')).toBeTruthy()
    fireEvent.click(screen.getByRole('button', { name: 'uploads.actions.retry' }))
    expect(retryUpload).toHaveBeenCalledWith('upload-1')
  })

  it.each([false, true])('preserves link copying and confirmed deletion for archived=%s', async archived => {
    const item = request({ finished: true })
    item.upload.status = 'completed'
    render(archived ? <ArchivedUploadList items={[item.upload]} /> : <UploadProgressList items={[item]} />)
    expect(screen.getByText('100%')).toBeTruthy()
    fireEvent.click(screen.getByRole('button', { name: 'uploads.actions.copy_link' }))
    expect(invoke).toHaveBeenCalledWith(archived ? 'copy_archived_upload_link' : 'copy_upload_link', { uploadId: 'upload-1' })
    fireEvent.click(screen.getByRole('button', { name: 'uploads.actions.delete' }))
    expect(vi.mocked(invoke).mock.calls.filter(([command]) => command.startsWith('delete'))).toHaveLength(0)
    fireEvent.click(within(screen.getByRole('dialog')).getByRole('button', { name: 'uploads.delete.confirm' }))
    expect(invoke).toHaveBeenCalledWith(archived ? 'delete_archived_upload' : 'delete_upload', { uploadId: 'upload-1' })
  })
})
