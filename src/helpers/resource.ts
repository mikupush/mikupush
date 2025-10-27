import { invoke } from '@tauri-apps/api/core'

export async function resourcePath(resource: string) {
  return invoke<string>('resource_path', { resource })
}
